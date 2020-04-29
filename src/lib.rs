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
        instance.load(0, 0x10);
        instance.load(1, 0x12);
        instance.load(2, 0x34);
        instance.load(3, 2);     
        instance.step();
        assert_eq!(instance.get_register("r1").unwrap(), 0x1234);
    }

    #[test]
    fn mov_reg_reg() {
        let mut instance = CPU::new(Memory::new(256));
        instance.load(0, 0x10);
        instance.load(1, 0x12);
        instance.load(2, 0x34);
        instance.load(3, 2);     
        instance.load(4, 0x11);
        instance.load(5, 2);
        instance.load(6, 3);
        instance.step();
        instance.step();
        assert_eq!(instance.get_register("r2").unwrap(), 0x1234);
    }

    // #[test]
    // fn mov_mem_reg() {

    // }

    // #[test]
    // fn mov_reg_mem() {
    //     assert_eq!(0, 0);
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