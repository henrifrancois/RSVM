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
        assert_eq!(0, 0);
    }

    #[test]
    fn mov_reg_reg() {
        assert_eq!(0, 0);
    }

    #[test]
    fn mov_mem_reg() {
        assert_eq!(0, 0);
    }

    #[test]
    fn mov_reg_mem() {
        assert_eq!(0, 0);
    }

    #[test]
    fn add_reg_reg() {
        assert_eq!(0, 0);
    }

    #[test]
    fn jmp_neq(){
        assert_eq!(0, 0);
    }
}