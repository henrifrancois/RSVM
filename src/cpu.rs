use std::collections::HashMap;

use crate::memory::*;
use crate::instructions::*;


const REGISTERS: &[&str] = &["ip", "acc", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8"];

#[derive(Debug)]
pub enum CPUError {
    LoadFailure,
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

    pub fn get_register(&self, name: &str) -> Result<u16, CPUError> {
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

    pub fn set_register(&mut self, name: &str, value: u16) -> Result<(), CPUError> {
        match self.reg_map.get(name) {
            Some(address_ref) => {
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

    fn get_register_index(&mut self) -> Result<u8, CPUError> {
        Ok((self.fetch().unwrap() % self.register_names.len() as u8) * 2 as u8)
    }

    fn fetch(&mut self) -> Result<u8, CPUError> {
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

    fn fetch16(&mut self) -> Result<u16, CPUError> {
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

    fn execute(&mut self, instruction: u8) -> Result<(), CPUError> {
        match instruction {
            // move a literal value into the r1 register
            MOV_LIT_REG => {
                let literal = self.fetch16();
                match literal {
                    Ok(literal) => {
                        let register = self.get_register_index().unwrap();
                        let [byte0, byte1] = literal.to_le_bytes();
                        self.registers[register as usize] = byte0;
                        self.registers[register as usize + 1] = byte1;
                        Ok(())
                    }
                    Err(_) => Err(CPUError::ExecutionFailure)
                }
            },
            MOV_REG_REG => {
                let from_register = self.get_register_index().unwrap();
                let to_register = self.get_register_index().unwrap();
                let byte0 = self.registers[from_register as usize];
                let byte1 = self.registers[from_register as usize + 1];
                self.registers[to_register as usize] = byte0;
                self.registers[to_register as usize + 1] = byte1;
                Ok(())
            },
            MOV_REG_MEM => {
                let register =self.get_register_index().unwrap();
                let address = self.fetch16().unwrap();
                let byte0 = self.registers[register as usize];
                let byte1 = self.registers[register as usize + 1];
                self.memory[address as usize] = byte0;
                self.memory[address as usize + 1] = byte1;
                Ok(())
            },
            MOV_MEM_REG => {
                let address = self.fetch16().unwrap();
                let register = self.get_register_index().unwrap();
                let byte0 = self.memory[address as usize];
                let byte1 = self.memory[address as usize + 1];
                self.registers[register as usize] = byte0;
                self.registers[register as usize + 1] = byte1;
                Ok(())
            },
            JMP_NEQ => {
                let value = self.fetch16().unwrap();
                let address = self.fetch16().unwrap();

                if value != self.get_register("acc").unwrap() {
                    self.set_register("ip", address);
                }
                Ok(())
            },
            ADD_REG_REG => {
                let r1 = self.fetch().unwrap(); // the result is the index of r1 in the registers array
                let r2 = self.fetch().unwrap(); // index of r2 in the registers array
                let reg_val_a0 = self.registers[r1 as usize * 2];
                let reg_val_b0 = self.registers[(r1 as usize * 2) + 1];
                // stich together the two to get our final value, while factoring in endianness
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

    pub fn load(&mut self, index: usize, value: u8) -> Result<(), CPUError> {
        self.memory[index] = value;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), CPUError> {
        let instruction = self.fetch().unwrap();
        self.execute(instruction)
    }

    pub fn display(&self) {
        for reg_name in &self.register_names {
            println!("{}:\t0x{:04x}", reg_name, self.get_register(reg_name).unwrap());
        }
        println!()
    }
}