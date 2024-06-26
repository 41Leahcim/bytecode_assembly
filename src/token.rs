use crate::{
    compile::{code::Code, error::Error},
    value::Value,
};
pub mod argument;

use serde::{Deserialize, Serialize};

use self::argument::read_reg_args;

#[derive(Debug, Serialize, Deserialize)]
pub enum Token {
    Comment(String),
    Out(String),
    Mov(u8, Value),
    Add(u8, Value, Value),
    Sub(u8, Value, Value),
    Mul(u8, Value, Value),
    Div(u8, Value, Value),
    Mod(u8, Value, Value),
    Label(String),
    Jmp(Label),
    Jl(Label),
    Jg(Label),
    Je(Label),
    Cmp(Value, Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Label {
    Base(String),
    Address(usize),
}

impl Token {
    /// Reads the arguments of the move operation and returns the operation with arguments
    pub fn mov(code: &mut Code) -> Result<Self, Error> {
        // Read the arguments
        let (register, arguments) = read_reg_args::<1>(code)?;

        // Return the instruction
        Ok(Self::Mov(register, arguments[0]))
    }

    /// Reads the add operation, returns the add operation with arguments
    pub fn add(code: &mut Code) -> Result<Self, Error> {
        // Read the arguments
        let (register, arguments) = read_reg_args::<2>(code)?;

        // Return the add operation
        Ok(Self::Add(register, arguments[0], arguments[1]))
    }

    /// Reads the sub operation, returns the sub operation with arguments
    pub fn sub(code: &mut Code) -> Result<Self, Error> {
        // Read the arguments
        let (register, arguments) = read_reg_args::<2>(code)?;

        // Return the sub operation
        Ok(Self::Sub(register, arguments[0], arguments[1]))
    }

    /// Reads the mul operation, returns the mul operation with arguments
    pub fn mul(code: &mut Code) -> Result<Self, Error> {
        // Read the arguments
        let (register, arguments) = read_reg_args::<2>(code)?;

        // Return the mul operation
        Ok(Self::Mul(register, arguments[0], arguments[1]))
    }

    /// Reads the div operation, returns the div operation with arguments
    pub fn div(code: &mut Code) -> Result<Self, Error> {
        // Read the arguments
        let (register, arguments) = read_reg_args::<2>(code)?;

        // Return the div operation
        Ok(Self::Div(register, arguments[0], arguments[1]))
    }

    /// Reads the mod operation, returns the mod operation with arguments
    pub fn modulo(code: &mut Code) -> Result<Self, Error> {
        // Read the arguments
        let (register, arguments) = read_reg_args::<2>(code)?;

        // Return the mod operation
        Ok(Self::Mod(register, arguments[0], arguments[1]))
    }
}
