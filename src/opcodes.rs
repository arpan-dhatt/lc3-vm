#![allow(clippy::upper_case_acronyms)]

use crate::utils::{sext, Condition};

macro_rules! bits {
    ($v:ident[$i:literal]) => {
        ($v >> $i) & 1
    };
    ($v:ident[$a:literal:$b:literal]) => {
        (($v >> $b) & ((2 << ($a - $b)) - 1)) as i16
    };
}

#[derive(Debug, PartialEq)]
pub enum Inst {
    ADD { dr: i16, sr1: i16, sr2: i16 },
    ADDi { dr: i16, sr1: i16, imm: i16 },
    AND { dr: i16, sr1: i16, sr2: i16 },
    ANDi { dr: i16, sr: i16, imm: i16 },
    BR { cond: Condition, pc_offset: i16 },
    JMP { base_r: i16 },
    JSR { pc_offset: i16 },
    JSRr { base_r: i16 },
    LD { dr: i16, pc_offset: i16 },
    LDI { dr: i16, pc_offset: i16 },
    LDR { dr: i16, base_r: i16, offset: i16 },
    LEA { dr: i16, pc_offset: i16 },
    NOT { dr: i16, sr: i16 },
    RTI,
    ST { sr: i16, pc_offset: i16 },
    STI { sr: i16, pc_offset: i16 },
    STR { sr: i16, base_r: i16, offset: i16 },
    TRAP { trap_vect: i16 },
}

impl From<u16> for Inst {
    fn from(raw: u16) -> Self {
        let op_code = bits!(raw[15:12]);
        match op_code {
            0b0001 => match bits!(raw[5]) {
                0 => Inst::ADD {
                    dr: bits!(raw[11:9]),
                    sr1: bits!(raw[8:6]),
                    sr2: bits!(raw[2:0]),
                },
                _ => Inst::ADDi {
                    dr: bits!(raw[11:9]),
                    sr1: bits!(raw[8:6]),
                    imm: sext(bits!(raw[4:0]), 5),
                },
            },
            0b0101 => match bits!(raw[5]) {
                0 => Inst::AND {
                    dr: bits!(raw[11:9]),
                    sr1: bits!(raw[8:6]),
                    sr2: bits!(raw[2:0]),
                },
                _ => Inst::ANDi {
                    dr: bits!(raw[11:9]),
                    sr: bits!(raw[8:6]),
                    imm: sext(bits!(raw[4:0]), 5),
                },
            },
            0b0000 => Inst::BR {
                cond: Condition {
                    n: bits!(raw[11]) == 1,
                    z: bits!(raw[10]) == 1,
                    p: bits!(raw[9]) == 1,
                },
                pc_offset: sext(bits!(raw[8:0]), 9),
            },
            0b1100 => Inst::JMP {
                base_r: bits!(raw[8:6]),
            },
            0b0100 => match bits!(raw[11]) {
                0 => Inst::JSRr {
                    base_r: bits!(raw[8:6]),
                },
                _ => Inst::JSR {
                    pc_offset: sext(bits!(raw[10:0]), 11),
                },
            },
            0b0010 => Inst::LD {
                dr: bits!(raw[11:9]),
                pc_offset: sext(bits!(raw[8:0]), 9),
            },
            0b1010 => Inst::LDI {
                dr: bits!(raw[11:9]),
                pc_offset: sext(bits!(raw[8:0]), 9),
            },
            0b0110 => Inst::LDR {
                dr: bits!(raw[11:9]),
                base_r: bits!(raw[8:6]),
                offset: sext(bits!(raw[5:0]), 6),
            },
            0b1110 => Inst::LEA {
                dr: bits!(raw[11:9]),
                pc_offset: sext(bits!(raw[8:0]), 9),
            },
            0b1001 => Inst::NOT {
                dr: bits!(raw[11:9]),
                sr: bits!(raw[8:6]),
            },
            0b1000 => Inst::RTI,
            0b0011 => Inst::ST {
                sr: bits!(raw[11:9]),
                pc_offset: sext(bits!(raw[8:0]), 9),
            },
            0b1011 => Inst::STI {
                sr: bits!(raw[11:9]),
                pc_offset: sext(bits!(raw[8:0]), 9),
            },
            0b0111 => Inst::STR {
                sr: bits!(raw[11:9]),
                base_r: bits!(raw[8:6]),
                offset: sext(bits!(raw[5:0]), 6),
            },
            0b1111 => Inst::TRAP {
                trap_vect: bits!(raw[7:0]),
            },
            _ => panic!("Illegal OpCode: {:b}", op_code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Condition, Inst};
    use crate::utils::sext;

    #[test]
    fn test_sext() {
        assert_eq!(sext(0b111111111, 9), !1 + 1);
        assert_eq!(sext(0b00001, 5), 1);
    }

    #[test]
    fn test_bits() {
        let a = 0b10;
        assert_eq!(bits!(a[1]), 1);
        let b = 0b11000;
        assert_eq!(bits!(b[4:3]), 3);
        let c = 0b1100_000_111_000000;
        assert_eq!(bits!(c[15:12]), 0b1100);
    }

    #[test]
    fn test_raw_to_inst_add() {
        let add_raw = 0b0001_000_001_0_00_010;
        let add = Inst::ADD {
            dr: 0,
            sr1: 1,
            sr2: 2,
        };
        assert_eq!(Inst::from(add_raw), add);
    }

    #[test]
    fn test_raw_to_inst_add_imm() {
        let add_imm_raw = 0b0001_111_011_1_10001;
        let add_imm = Inst::ADDi {
            dr: 7,
            sr1: 3,
            imm: -15,
        };
        assert_eq!(Inst::from(add_imm_raw), add_imm);
    }

    #[test]
    fn test_raw_to_inst_and() {
        let and_raw = 0b0101_011_000_0_00_111;
        let and = Inst::AND {
            dr: 3,
            sr1: 0,
            sr2: 7,
        };
        assert_eq!(Inst::from(and_raw), and);
    }

    #[test]
    fn test_raw_inst_and_imm() {
        let and_imm_raw = 0b0101_110_000_1_11011;
        let and_imm = Inst::ANDi {
            dr: 6,
            sr: 0,
            imm: -5,
        };
        assert_eq!(Inst::from(and_imm_raw), and_imm);
    }

    #[test]
    fn test_raw_to_inst_br() {
        let br_raw = 0b0000_110_010010010;
        let br = Inst::BR {
            cond: Condition {
                n: true,
                z: true,
                p: false,
            },
            pc_offset: 146,
        };
        assert_eq!(Inst::from(br_raw), br);
    }

    #[test]
    fn test_raw_to_inst_jmp() {
        let jmp_raw = 0b1100_000_111_000000;
        let jmp = Inst::JMP { base_r: 7 };
        assert_eq!(Inst::from(jmp_raw), jmp);
    }

    #[test]
    fn test_raw_to_inst_jsr() {
        let jsr_raw = 0b0100_1_01001001011;
        let jsr = Inst::JSR { pc_offset: 587 };
        assert_eq!(Inst::from(jsr_raw), jsr);
    }

    #[test]
    fn test_raw_to_inst_jsrr() {
        let jsrr_raw = 0b0100_0_00_011_000000;
        let jsrr = Inst::JSRr { base_r: 3 };
        assert_eq!(Inst::from(jsrr_raw), jsrr);
    }

    #[test]
    fn test_raw_to_inst_ld() {
        let ld_raw = 0b0010_101_111000111;
        let ld = Inst::LD {
            dr: 5,
            pc_offset: sext(0b111000111, 9),
        };
        assert_eq!(Inst::from(ld_raw), ld);
    }

    #[test]
    fn test_raw_to_inst_ldi() {
        let ldi_raw = 0b1010_001_011000110;
        let ldi = Inst::LDI {
            dr: 1,
            pc_offset: sext(0b011000110, 9),
        };
        assert_eq!(Inst::from(ldi_raw), ldi);
    }

    #[test]
    fn test_raw_to_inst_ldr() {
        let ldr_raw = 0b0110_110_001_000000;
        let ldr = Inst::LDR {
            dr: 6,
            base_r: 1,
            offset: 0,
        };
        assert_eq!(Inst::from(ldr_raw), ldr);
    }

    #[test]
    fn test_raw_to_inst_lea() {
        let lea_raw = 0b1110_000_111111111;
        let lea = Inst::LEA {
            dr: 0,
            pc_offset: -1,
        };
        assert_eq!(Inst::from(lea_raw), lea);
    }

    #[test]
    fn test_raw_to_inst_not() {
        let not_raw = 0b1001_000_111_1_11111;
        let not = Inst::NOT { dr: 0, sr: 7 };
        assert_eq!(Inst::from(not_raw), not);
    }

    #[test]
    fn test_raw_to_inst_st() {
        let st_raw = 0b0011_000_111111111;
        let st = Inst::ST {
            sr: 0,
            pc_offset: -1,
        };
        assert_eq!(Inst::from(st_raw), st);
    }

    #[test]
    fn test_raw_to_inst_sti() {
        let sti_raw = 0b1011_010_111111110;
        let sti = Inst::STI {
            sr: 2,
            pc_offset: -2,
        };
        assert_eq!(Inst::from(sti_raw), sti);
    }

    #[test]
    fn test_raw_to_inst_str() {
        let str_raw = 0b0111_010_001_111111;
        let _str = Inst::STR {
            sr: 2,
            base_r: 1,
            offset: -1,
        };
        assert_eq!(Inst::from(str_raw), _str);
    }

    #[test]
    fn test_raw_to_inst_trap() {
        let trap_raw = 0b1111_0000_00110011;
        let trap = Inst::TRAP { trap_vect: 0x33 };
        assert_eq!(Inst::from(trap_raw), trap);
    }
}
