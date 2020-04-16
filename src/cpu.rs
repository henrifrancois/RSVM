use std::collections::HashMap;

use crate::memory::*;

const REGISTERS: &'static [&'static str] = &["ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8"];

enum CPUError {
    InvalidRegister
}

pub struct CPU {
    memory: Memory,
    register_names: Vec<&'static str>,
    registers: Memory,
    reg_map: HashMap<&'static str, usize>
}

impl CPU {
    pub fn new(memoryItem: Memory) -> Self {
        let mut aggr: HashMap<&'static str, usize> = HashMap::new();
        let memsize = REGISTERS.to_vec().len() * 2;
        let mut offsets: Vec<usize> = Vec::new();
        for i in 0..REGISTERS.to_vec().len() + 1 {
            offsets.push(i * 2);
        }
        let pairs: Vec<_> = REGISTERS.iter().zip(offsets.iter()).collect();
        for mapping in pairs {
            aggr.insert(*mapping.0, *mapping.1);
        }
        CPU {
            memory: memoryItem,
            register_names: REGISTERS.to_vec(),
            registers: Memory::new(memsize),
            reg_map: aggr,
        }
    }

    fn get_register(&self, name: &str) -> Result<usize, CPUError> {
        match self.reg_map.get(name) {
            Some(value) => {
                let register_value = *value;        
                // here we have value = the offset from the start of the reg_map
                // we can enter this offset into the registers array to get the data present there
                Ok(self.registers[register_value])
            }
            None => {
                Err(CPUError::InvalidRegister)
            }
        }
    }

    fn set_register(&mut self, name: &str, value: usize) -> Result<(), CPUError> {
        match self.reg_map.get(name) {
            Some(reg) => {
                self.registers[*reg] = value;
                // We have the offset which is captured by the offhand `reg` variable.
                // The value can now be entered into this register by simple assignment
                Ok(())
            }
            None => {
                Err(CPUError::InvalidRegister)
            }
        }
    }

    pub fn fetch(&mut self) -> usize {
        let nextInstAddr = self.get_register('ip');
        let instruction = self.memory[nextInstAddr];
        self.set_register('ip', nextInstAddr + 1);
        return instruction;
    }

    pub fn fetch16(&mut self) -> usize {
        let nextInstAddr = self.get_register('ip');
        let instruction = self.memory[nextInstAddr];
        self.set_register('ip', nextInstAddr + 1);
        return instruction;
    }    

    pub fn execute(instruction: usize) {
        match instruction {
            0x10 => {

            },

        }
    }
}