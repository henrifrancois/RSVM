mod instructions;
mod memory;
mod cpu;

use memory::*;
use cpu::CPU;
use instructions::*;


fn main() {
    let mut mem = Memory::new(256);
    
    mem[0] = MOV_LIT_R1;
    mem[1] = 0x12;
    mem[2] = 0x34;

    // mem[3] = MOV_LIT_R2;
    // mem[4] = 0xAB;
    // mem[5] = 0xCD;

    let mut cpu = CPU::new(mem);
    
    cpu.display();
    
    cpu.step();
    cpu.display();

    // cpu.step();
    // cpu.display();
}
