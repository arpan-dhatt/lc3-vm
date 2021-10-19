use lc3_tools::{LC3, opcodes::Inst};


fn main() {
    let mut vm = LC3::default();
    vm.memory[0x300A] = 'h' as u16;
    vm.memory[0x300B] = 'i' as u16;
    vm.memory[0x300C] = '\n' as u16;
    vm.memory[0x300D] = '\0' as u16;
    
    vm.memory[0x3000] = 0b1110_000_000001001; // LEA R0 <- 0x300A
    vm.memory[0x3001] = 0b1111_0000_00100010; // PUTS
    vm.memory[0x3002] = 0b1111_0000_00100011; // IN
    vm.memory[0x3003] = 0b1111_0000_00100001; // OUT
    vm.memory[0x3004] = 0b1111_0000_00100101; // HALT

    vm.run();
}
