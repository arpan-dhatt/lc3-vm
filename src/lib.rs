mod opcodes;
mod utils;

pub struct LC3 {
    memory: [u16; 65536],
    registers: [i16; 8],
    pc: u16,
    supervisor: bool,
    priority: u8,
    condition: utils::Condition,
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
