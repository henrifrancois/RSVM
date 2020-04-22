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
    pub fn new(memory_item: Memory) -> Self {
        let mut aggr: HashMap<&'static str, u8> = HashMap::new();
        let mut offsets: Vec<u8> = Vec::new();
        let memsize = REGISTERS.to_vec().len() * 2;
        for i in 0..REGISTERS.to_vec().len() + 1 {
            offsets.push(i as u8 * 2);
        }
        let pairs: Vec<_> = REGISTERS.iter().zip(offsets.iter()).collect();
        for mapping in pairs {
            aggr.insert(*mapping.0, *mapping.1);
        }
        CPU {
            memory: memory_item,
            register_names: REGISTERS.to_vec(),
            registers: Memory::new(memsize),
            reg_map: aggr,
        }
    }

    fn get_register(&self, name: &str) -> Result<u16, CPUError> {
        match self.reg_map.get(name) {
            Some(address_ref) => {
                // here we have address_ref = the offset from the start of the reg_map
                let register_start_address = *address_ref as usize;       
                // we can enter this offset into the registers array to get the data present there
                let byte0 = self.registers[register_start_address];
                let byte1 = self.registers[register_start_address + 1];
                let register = ((byte1 as u16) << 8) | byte0 as u16;
                Ok(register)
            }
            None => {
                Err(CPUError::InvalidRegister)
            }
        }
    }

    fn set_register(&mut self, name: &str, value: u16) -> Result<(), CPUError> {
        match self.reg_map.get(name) {
            Some(address_ref) => {
                println!("Value: 0x{:04x}", value);
                // the value being set is 16 bits long, while each entry in our registers array is 8 bits long
                // as each register spans two 8 bit values, we need to break down our 16 bit value into two 8 bit values.
                let [byte0, byte1]: [u8; 2] = value.to_ne_bytes();
                // We have the offset which is captured by the offhand `address_ref` variable.
                // The value can now be entered into this register by simple assignment
                self.registers[(*address_ref) as usize] = byte0;
                self.registers[(*address_ref) as usize + 1] = byte1;
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
                let [byte0, _byte1]: [u8; 2] = address.to_ne_bytes();
                let instruction = self.memory[byte0 as usize];
                self.set_register("ip", address + 1).unwrap();
                Ok(instruction)
            }
            Err(_) => Err(CPUError::FetchFailure)
        }
    }

    pub fn fetch16(&mut self) -> Result<u16, CPUError> {
        // fetch16 looks into the Instruction Pointer's address and from there, 
        // gets the value in the pointed to register.
        let first_inst = self.fetch();
        match first_inst {
            Ok(instruction0) =>  {
                let inst0 = instruction0;
                let second_inst = self.fetch();
                match second_inst {
                    Ok(instruction1) => {
                        let inst1 = instruction1;
                        println!("First instruction: 0x{:04x}", inst0);
                        println!("Second instruction: 0x{:04x}", inst1);
                        let instruction = ((inst0 as u16) << 8) | inst1 as u16;
                        Ok(instruction)
                    },
                    Err(_) => {
                        Err(CPUError::FetchFailure)
                    }
                }
            },
            Err(_) => {
                Err(CPUError::FetchFailure)
            }
        }
    }

    pub fn execute(&mut self, instruction: u8) -> Result<(), CPUError> {
        match instruction {
            // move a literal value into the r1 register
            MOV_LIT_R1 => {
                let literal = self.fetch16();
                match literal {
                    Ok(literal) => {
                        self.set_register("r1", literal).unwrap();
                        Ok(())
                    }
                    Err(_) => Err(CPUError::ExecutionFailure)
                }
            },
            MOV_LIT_R2 => {
                let literal = self.fetch16();
                match literal {
                    Ok(literal) => {
                        self.set_register("r2", literal).unwrap();
                        Ok(())
                    }
                    Err(_) => Err(CPUError::ExecutionFailure)
                }
            },
            ADD_REG_REG => {
                let r1 = self.fetch().unwrap();
                let r2 = self.fetch().unwrap();
                let reg_val_a0 = self.registers[r1 as usize * 2];
                let reg_val_b0 = self.registers[(r1 as usize * 2) + 1];
                let reg_val0  = ((reg_val_b0 as u16) << 8) | reg_val_a0 as u16;

                let reg_val_a1 = self.registers[r2 as usize * 2];
                let reg_val_b1 = self.registers[(r2 as usize * 2) + 1];
                let reg_val1  = ((reg_val_b1 as u16) << 8) | reg_val_a1 as u16;

                self.set_register("acc", reg_val0 + reg_val1).unwrap();
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