use std::collections::HashMap;

use crate::memory::*;
use crate::instructions::*;


const REGISTERS: &'static [&'static str] = &["ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8"];

#[derive(Debug)]
pub enum CPUError {
    InvalidRegister,
    FetchFailure,
    ExecutionFailure,
    InvalidInstruction
}

pub struct CPU {
    pub memory: Memory,
    register_names: Vec<&'static str>,
    registers: Memory,
    reg_map: HashMap<&'static str, u8>
}


impl CPU {
    pub fn new(memoryItem: Memory) -> Self {
        let mut aggr: HashMap<&'static str, u8> = HashMap::new();
        let memsize = REGISTERS.to_vec().len() * 2;
        let mut offsets: Vec<u8> = Vec::new();
        for i in 0..REGISTERS.to_vec().len() + 1 {
            offsets.push(i as u8 * 2);
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

    fn get_register(&self, name: &str) -> Result<u16, CPUError> {
        match self.reg_map.get(name) {
            Some(value) => {
                let register_value = *value as usize;       
                // here we have value = the offset from the start of the reg_map
                // we can enter this offset into the registers array to get the data present there
                let byte0 = self.registers[register_value];
                let byte1 = self.registers[register_value+1];
                let register = ((byte0 as u16) << 8) | byte1 as u16;
                Ok(register)
            }
            None => {
                Err(CPUError::InvalidRegister)
            }
        }
    }

    fn set_register(&mut self, name: &str, value: u16) -> Result<(), CPUError> {
        match self.reg_map.get(name) {
            Some(reg) => {
                println!("Value to be set: 0x{:04x}", value);
                let [byte0, byte1]: [u8; 2] = value.to_be_bytes();
                println!("Pieces of value: 0x{:04x}, 0x{:04x}", byte0, byte1);
                self.registers[(*reg) as usize] = byte0;
                self.registers[(*reg) as usize + 1] = byte1;
                // We have the offset which is captured by the offhand `reg` variable.
                // The value can now be entered into this register by simple assignment
                Ok(())
            }
            None => {
                Err(CPUError::InvalidRegister)
            }
        }
    }

    pub fn fetch(&mut self) -> Result<u8, CPUError> {
        // fetch looks into the Instruction Pointer's address and from there, 
        // gets the value in the pointed to register's first byte
        let instruction_addr = self.get_register("ip");     // get_register returns a 16bit address. We need 8 bits to referece our memory
        match instruction_addr {
            Ok(address) => {
                // We retrieve the first byte of the given address
                let [byte0, byte1]: [u8; 2] = address.to_be_bytes();
                println!("fetch() address: 0x{:04x}\tbytes: 0x{:04x}", address, byte0);
                let instruction = self.memory[byte0 as usize];
                self.set_register("ip", address + 1);
                
                Ok(instruction)
            }
            Err(_) => Err(CPUError::FetchFailure)
        }
    }

    pub fn fetch16(&mut self) -> Result<u16, CPUError> {
        // fetch16 looks into the Instruction Pointer's address and from there, 
        // gets the value in the pointed to register.
        let instruction_addr = self.get_register("ip");
        match instruction_addr{
            Ok(address) => {
                println!("Address of IP: 0x{:04x}", address);
                let [byte0, byte1]: [u8; 2] = address.to_be_bytes();

                let inst0 = self.memory[byte0 as usize];
                let inst1 = self.memory[byte1 as usize];

                println!("Instructions -> 1: 0x{:04x}\t2: 0x{:04x}", inst0, inst1);
                 
                let instruction = ((inst0 as u16) << 8) | inst1 as u16;

                self.set_register("ip", address + 2);
                println!("New address of IP: 0x{:04x}", self.get_register("ip").unwrap());
                Ok(instruction)
            }
            Err(_) => Err(CPUError::FetchFailure)
        }
    }

    pub fn execute(&mut self, instruction: u8) -> Result<(), CPUError> {
        match instruction {
            // move a literal value into the r1 register
            MOV_LIT_R1 => {
                let literal = self.fetch16();
                match literal {
                    Ok(literal) => {
                        println!("Value of literal: 0x{:04x}", literal);
                        self.set_register("r1", literal);
                        Ok(())
                    }
                    Err(_) => Err(CPUError::ExecutionFailure)
                }
            },
            MOV_LIT_R2 => {
                let literal = self.fetch16();
                match literal {
                    Ok(literal) => {
                        self.set_register("r2", literal);
                        Ok(())
                    }
                    Err(_) => Err(CPUError::ExecutionFailure)
                }
            },
            ADD_REG_REG => {
                let r1 = self.fetch().unwrap();
                let r2 = self.fetch().unwrap();
                let reg_val0A = self.registers[r1 as usize * 2];
                let reg_val0B = self.registers[(r1 as usize + 1) * 2];
                let reg_val0  = ((reg_val0A as u16) << 8) | reg_val0B as u16;

                let reg_val1A = self.registers[r2 as usize* 2];
                let reg_val1B = self.registers[(r2 as usize + 1) * 2];
                let reg_val1  = ((reg_val1A as u16) << 8) | reg_val1B as u16;

                self.set_register("acc", reg_val0 + reg_val1);
                Ok(())

            },
            _ => Err(CPUError::InvalidInstruction)
        }
    }

    pub fn step(&mut self) -> Result<(), CPUError> {
        let instruction = self.fetch().unwrap();
        self.execute(instruction)
    }

    pub fn display(&self) {
        for reg_name in &self.register_names {
            println!("{}:\t0x{:04x}", reg_name, self.get_register(reg_name).unwrap());
        }
        println!("")
    }
}