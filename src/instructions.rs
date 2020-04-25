// All instruction definitions
#[allow(dead_code)]
pub const MOV_LIT_REG: u8 = 0x10;   // Move a value into a register
pub const MOV_REG_REG: u8 = 0x11;   // Move a register's value to another register
pub const MOV_REG_MEM: u8 = 0x12;   // Move a value from a register to memory
pub const MOV_MEM_REG: u8 = 0x13;   // Move a value from memory to a register
pub const ADD_REG_REG: u8 = 0x14;   // Add values from two registers to the acc register
pub const JMP_NEQ: u8 = 0x15;       // Jump if the value in the accumulator is different from a literal
