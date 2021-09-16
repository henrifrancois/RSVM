mod instructions;
mod memory;
mod cpu;

use memory::*;
use cpu::CPU;
use instructions::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mov_lit_reg() {
        let mut instance = CPU::new(Memory::new(256));
        instance.load(0, 0x10); // instruction
        instance.load(1, 0x12); // byte0 literal
        instance.load(2, 0x34); // byte1 literal
        instance.load(3, 2);     // register "R1"
        instance.step();
        assert_eq!(instance.get_register("r1").unwrap(), 0x1234);
    }

    #[test]
    fn mov_reg_reg() {
        let mut instance = CPU::new(Memory::new(256));
        instance.load(0, 0x10); // instruction MOV LIT REG
        instance.load(1, 0x12); // byte0 literal
        instance.load(2, 0x34); // byte1 literal
        instance.load(3, 2);    // register "R1"
        instance.load(4, 0x11); // instruction MOV REG REG
        instance.load(5, 2);    // register "R1"
        instance.load(6, 3);    // register "R2"
        instance.step();
        instance.step();
        assert_eq!(instance.get_register("r2").unwrap(), 0x1234);
    }

    #[test]
    fn mov_reg_mem() {
        let mut instance = CPU::new(Memory::new(256));
        instance.load(0, 0x10); // instruction MOV LIT REG
        instance.load(1, 0x12); // byte0 literal
        instance.load(2, 0x34); // byte1 literal
        instance.load(3, 2);    // register "R1"

        assert_eq!(0, 0);
    }

    // #[test]
    // fn mov_mem_reg() {

    // }

    // #[test]
    // fn add_reg_reg() {
    //     assert_eq!(0, 0);
    // }

    // #[test]
    // fn jmp_neq(){
    //     assert_eq!(0, 0);
    // }
}