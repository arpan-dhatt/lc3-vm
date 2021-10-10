mod opcodes;
mod utils;

use std::io::{Read, Write};

use opcodes::Inst;

pub struct LC3 {
    pub memory: [u16; 65536],
    pub registers: [i16; 8],
    pub pc: u16,
    pub supervisor: bool,
    pub priority: u8,
    pub condition: utils::Condition,
    pub halted: bool,
}

impl LC3 {
    pub fn run_instruction(&mut self, inst: Inst) {
        macro_rules! reg {
            [$v:expr] => {
                self.registers[$v as usize]
            }
        }

        macro_rules! mem {
            [r, $v:expr] => {
                self.memory[$v as usize] as i16
            };
            [w, $v:expr] => {
                self.memory[$v as usize]
            }
        }

        match inst {
            Inst::ADD { dr, sr1, sr2 } => {
                reg![dr] = reg![sr1].wrapping_add(reg![sr2]);
                self.set_condition(reg![dr])
            }
            Inst::ADDi { dr, sr, imm } => {
                reg![dr] = reg![sr].wrapping_add(imm);
                self.set_condition(reg![dr]);
            }
            Inst::AND { dr, sr1, sr2 } => {
                reg![dr] = reg![sr1] & reg![sr2];
                self.set_condition(reg![dr])
            }
            Inst::ANDi { dr, sr, imm } => {
                reg![dr] = reg![sr] & imm;
                self.set_condition(reg![dr])
            }
            Inst::BR { cond, pc_offset } => {
                if self.condition.is_satisfied_by(&cond) {
                    self.pc = self.pc.wrapping_add(pc_offset as u16);
                }
            }
            Inst::JMP { base_r } => {
                self.pc = self.pc.wrapping_add(reg![base_r] as u16);
            }
            Inst::JSR { pc_offset } => {
                reg![7] = self.pc as i16;
                self.pc = self.pc.wrapping_add(pc_offset as u16);
            }
            Inst::JSRr { base_r } => {
                reg![7] = self.pc as i16;
                self.pc = reg![base_r] as u16;
            }
            Inst::LD { dr, pc_offset } => {
                reg![dr] = mem![r, self.pc.wrapping_add(pc_offset as u16)];
                self.set_condition(reg![dr]);
            }
            Inst::LDI { dr, pc_offset } => {
                let mem_loc = self.pc.wrapping_add(pc_offset as u16);
                reg![dr] = mem![r, mem_loc];
                self.set_condition(reg![dr]);
            }
            Inst::LDR { dr, base_r, offset } => {
                reg![dr] = mem![r, reg![base_r].wrapping_add(offset)];
                self.set_condition(reg![dr]);
            }
            Inst::LEA { dr, pc_offset } => {
                reg![dr] = self.pc.wrapping_add(pc_offset as u16) as i16;
                self.set_condition(reg![dr]);
            }
            Inst::NOT { dr, sr } => {
                reg![dr] = !reg![sr];
                self.set_condition(reg![dr]);
            }
            Inst::RTI => {
                unimplemented!();
            }
            Inst::ST { sr, pc_offset } => {
                mem![w, self.pc.wrapping_add(pc_offset as u16)] = reg![sr] as u16;
            }
            Inst::STI { sr, pc_offset } => {
                mem![w, mem![r, self.pc.wrapping_add(pc_offset as u16)]] = reg![sr] as u16;
            }
            Inst::STR { sr, base_r, offset } => {
                mem![w, reg![base_r].wrapping_add(offset)] = reg![sr] as u16;
            }
            Inst::TRAP { trap_vect } => match trap_vect {
                0x20 => {
                    let mut c = [0; 1];
                    std::io::stdin().lock().read_exact(&mut c).unwrap();
                    reg![0] = c[0] as i16;
                },
                0x21 => {
                    let mut c = [0; 1];
                    c[0] = reg![0] as u8;
                    std::io::stdout().lock().write(&c).unwrap();
                },
                0x22 => {
                    let mut buf = Vec::new();
                    let mut spot = reg![0] as u16;
                    while mem![r, spot] != 0x0000 {
                        buf.push(mem![r, spot] as u8);
                        spot += 1;
                    }
                    std::io::stdout().lock().write_all(&buf).unwrap();
                },
                0x25 => self.halted = true,
                _ => unimplemented!(),
            },
        };
    }

    pub fn run_step(&mut self) {
        let inst = Inst::from(self.memory[self.pc as usize]);
        self.pc = self.pc.wrapping_add(1);
        self.run_instruction(inst);
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.run_step();
        }
    }

    fn set_condition(&mut self, val: i16) {
        self.condition.n = val < 0;
        self.condition.z = val == 0;
        self.condition.p = val > 0;
    }
}

impl Default for LC3 {
    fn default() -> Self {
        LC3 {
            memory: [0; 65536],
            registers: [0; 8],
            pc: 0x3000,
            supervisor: false,
            priority: 0,
            condition: utils::Condition::default(),
            halted: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{opcodes::Inst, utils::Condition, LC3};

    const lab1part1: [u16; 19] = [
        0b0010_000_011111111,   // loads X to R0
        0b0010_001_011111111,   // loads Y to R1
        0b0101_011_011_1_00000, // clears R3
        0b1001_001_001_111111,  // makes R1 negative
        0b0001_001_001_1_00001,
        0b0001_011_000_0_00_001, // ADDS R0 to R1 and stores in R3
        0b0000_010_000000010,    // branches if zero
        0b0000_001_000000011,    // branches if positive
        0b0000_100_000000110,    // branches if negative
        0b0011_011_011111000,    // stores 0 to x3102
        0b1111_0000_00100101,    // halt
        0b0101_011_011_1_00000,  // clears R3
        0b1001_011_011_111111,   // changes R3 to -1
        0b0011_011_011110100,    // stores -1 to x3102
        0b1111_0000_00100101,    // halt
        0b0101_011_011_1_00000,  // clears R3
        0b0001_011_011_1_00001,  // ADDs one to R3
        0b0011_011_011110000,    // stores 1 to x3102
        0b1111_0000_00100101,    // halt
    ];

    const lab1part2: [u16; 25] = [
        0b0101_001_001_1_00000,  // clears R1
        0b0101_100_100_1_00000,  // clears R4
        0b0001_100_100_1_01111,  // sets R4 to 15
        0b0001_100_100_1_00001,  // sets R4 to 16
        0b0010_000_011111011,    // loads x3100 to R0
        0b0001_000_000_1_00000,  // sets R0 to 0
        0b0000_010_000001111,    // branch
        0b0000_001_000000001,    // branch
        0b0000_100_000000111,    // branch
        0b0001_001_001_1_00001,  // ADD 1 to R1
        0b0001_000_000_0_00_000, // double R0
        0b0001_100_100_1_11111,  // decrement R4 by 1
        0b0000_001_111111000,    // branch
        0b0001_001_001_1_00000,  // decrement R1
        0b0011_001_011110010,    // store R1 to 0x3101
        0b1111_0000_00100101,    // halt
        0b0001_000_000_0_00_000, //double R0
        0b0001_100_100_1_11111,  // decrement R4
        0b0000_001_111110010,    // branch
        0b0001_001_001_1_00000,  // decrement R1
        0b0011_001_011101100, // store R1
        0b1111_0000_00100101,    // halt
        0b0001_001_100_0_00_001, // ADD R4 to R1
        0b0011_001_011101001,    // store R1
        0b1111_0000_00100101,    // halt
    ];

    #[test]
    fn test_run_ee306_lab_1_part_1_tc_1() {
        let mut lc3 = load_lc3(LC3::default(), &lab1part1, 0x3000);
        lc3.memory[0x3100] = 12;
        lc3.memory[0x3101] = 10;
        lc3.run();
        assert_eq!(lc3.memory[0x3102], 0xFFFF);
    }

    #[test]
    fn test_run_ee306_lab_1_part_1_tc_2() {
        let mut lc3 = load_lc3(LC3::default(), &lab1part1, 0x3000);
        lc3.memory[0x3100] = !1 + 1;
        lc3.memory[0x3101] = !1 + 1;
        lc3.run();
        assert_eq!(lc3.memory[0x3102], 0x0000);
    }

    #[test]
    fn test_run_ee306_lab_1_part_1_tc_3() {
        let mut lc3 = load_lc3(LC3::default(), &lab1part1, 0x3000);
        lc3.memory[0x3100] = !10 + 1;
        lc3.memory[0x3101] = 10;
        lc3.run();
        assert_eq!(lc3.memory[0x3102], 0x0001);
    }

    #[test]
    fn test_run_ee306_lab_1_part_2_tc_1() {
        let mut lc3 = load_lc3(LC3::default(), &lab1part2, 0x3000);
        lc3.memory[0x3100] = 0xFFFF;
        lc3.run();
        assert_eq!(lc3.memory[0x3101], 0);
    }

    #[test]
    fn test_run_ee306_lab_1_part_2_tc_2() {
        let mut lc3 = load_lc3(LC3::default(), &lab1part2, 0x3000);
        lc3.memory[0x3100] = 0xF0FF;
        lc3.run();
        for i in 0x3000..lc3.pc as usize {
            println!(
                "{:04x}: {:08b} {}",
                i,
                lc3.memory[i],
                Inst::from(lc3.memory[i])
            );
        }

        assert_eq!(lc3.memory[0x3101], 4);
    }

    #[test]
    fn test_run_ee306_lab_1_part_2_tc_3() {
        let mut lc3 = load_lc3(LC3::default(), &lab1part2, 0x3000);
        lc3.memory[0x3100] = 0x0000;
        lc3.run();
        assert_eq!(lc3.memory[0x3101], 16);
    }

    fn load_lc3(mut vm: LC3, code: &[u16], start: usize) -> LC3 {
        for i in 0..code.len() {
            vm.memory[start + i] = code[i];
        }
        vm
    }
}
