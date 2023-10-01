use crate::{value::Value, Token};
use std::{
    fmt::Write as _,
    io::{self, Write as _},
};

/// Executes the output command
pub fn out(output: &str, registers: &[Value]) {
    // Create a string for the result and registers
    let mut result = String::new();
    let mut register = String::new();

    // Create an iterator over the characters
    let mut output = output.chars();
    while let Some(c) = output.next() {
        // If the current character is a '{', it's probably meant for output
        if c == '{' {
            // Find '}' or another '{'
            let mut last_char = c;
            for c in output.by_ref() {
                if c == '}' || c == '{' {
                    last_char = c;
                    break;
                }
                register.push(c);
                last_char = c;
            }

            // If the last char was '{', it is meant to be printed as '{'
            if last_char == '{' {
                result.push('{');
            } else {
                // Otherwise, it's probably a register so print it
                // If it's not a valid register, print the read characters
                match register.parse::<u8>() {
                    Ok(index) => write!(&mut result, "{}", registers[index as usize]).unwrap(),
                    Err(_) => write!(&mut result, "{{{register}").unwrap(),
                }
            }

            // Clear the register buffer
            register.clear();
        } else {
            // Push the current character
            result.push(c);
        }
    }

    // Print the output ot the screen
    io::stdout().lock().write_all(result.as_bytes()).unwrap();
}

/// Executes the tokens
pub fn execute(tokens: &[Token]) {
    // Create and initialize the registers
    let mut registers = [Value::Number(0); 256];

    // Iterate through the tokens
    for token in tokens {
        // Execute the current token
        match token {
            Token::Comment(_) => {}
            Token::Out(output) => out(output, &registers),
            Token::Mov(id, value) => registers[*id as usize] = value.take(&registers),
            Token::Add(id, value, value2) => {
                registers[*id as usize] = value.add(value2, &registers)
            }
            Token::Sub(id, value, value2) => {
                registers[*id as usize] = value.sub(value2, &registers)
            }
        }
    }
}
