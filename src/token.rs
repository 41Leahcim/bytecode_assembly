use crate::{
    compile::{code::Code, error::Error},
    value::Value,
};
pub mod argument;
use argument::read_arguments;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Token {
    Comment(String),
    Out(String),
    Mov(u8, Value),
    Add(u8, Value, Value),
    Sub(u8, Value, Value),
    Mul(u8, Value, Value),
    Div(u8, Value, Value),
}

impl Token {
    /// Reads the arguments of the move operation and returns the operation with arguments
    pub fn mov(code: &mut Code) -> Result<Token, Error> {
        // Read the arguments
        let (register, arguments) = read_arguments::<1>(code)?;

        // Return the instruction
        Ok(Token::Mov(register, arguments[0]))
    }

    /// Reads the add operation, returns the add operation with arguments
    pub fn add(code: &mut Code) -> Result<Token, Error> {
        // Read the arguments
        let (register, arguments) = read_arguments::<2>(code)?;

        // Return the add operation
        Ok(Token::Add(register, arguments[0], arguments[1]))
    }

    /// Reads the sub operation, returns the sub operation with arguments
    pub fn sub(code: &mut Code) -> Result<Token, Error> {
        // Read the arguments
        let (register, arguments) = read_arguments::<2>(code)?;

        // Return the sub operation
        Ok(Token::Sub(register, arguments[0], arguments[1]))
    }

    /// Reads the mul operation, returns the mul operation with arguments
    pub fn mul(code: &mut Code) -> Result<Token, Error> {
        // Read the arguments
        let (register, arguments) = read_arguments::<2>(code)?;

        // Return the mul operation
        Ok(Token::Mul(register, arguments[0], arguments[1]))
    }

    /// Reads the div operation, returns the div operation with arguments
    pub fn div(code: &mut Code) -> Result<Token, Error> {
        // Read the arguments
        let (register, arguments) = read_arguments::<2>(code)?;

        // Return the mul operation
        Ok(Token::Div(register, arguments[0], arguments[1]))
    }
}
