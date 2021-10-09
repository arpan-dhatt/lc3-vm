mod opcodes;
mod utils;

use opcodes::Inst;

pub struct LC3 {
    memory: [u16; 65536],
    registers: [i16; 8],
    pc: u16,
    supervisor: bool,
    priority: u8,
    condition: utils::Condition,
}

impl LC3 {
    fn run_instruction(&mut self, inst: Inst) {
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
            Inst::ADDi { dr, sr1, imm } => {
                reg![dr] = reg![sr1].wrapping_add(imm);
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
            Inst::TRAP { trap_vect: _ } => {
                unimplemented!();
            }
        };
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
        }
    }
}
