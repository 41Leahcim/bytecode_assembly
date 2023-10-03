use crate::{token::Label, value::Value, Token};
use std::{
    collections::HashMap,
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

fn read_labels(tokens: &[Token]) -> HashMap<String, usize> {
    tokens
        .iter()
        .enumerate()
        .filter_map(|(index, label)| {
            if let Token::Label(label) = label {
                Some((label.to_owned(), index))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>()
}

/// Executes the tokens
pub fn execute(tokens: &[Token], mut cycles: Option<usize>) {
    // Create and initialize the registers
    let mut registers = [Value::Number(0); 256];
    let labels = read_labels(tokens);
    let mut index = 0;

    // Iterate through the tokens
    while let Some(token) = tokens.get(index) {
        cycles = cycles.map(|cycles| cycles - 1);
        if cycles.is_some_and(|cycles| cycles == 0) {
            break;
        }

        // Execute the current token
        match token {
            Token::Comment(_) => {}
            Token::Out(output) => out(output, &registers),
            Token::Mov(id, value) => registers[*id as usize] = value.take(&registers),
            Token::Add(id, value, value2) => {
                registers[*id as usize] =
                    value.perform_operation(value2, &registers, i64::wrapping_add)
            }
            Token::Sub(id, value, value2) => {
                registers[*id as usize] =
                    value.perform_operation(value2, &registers, i64::wrapping_sub)
            }
            Token::Mul(id, value, value2) => {
                registers[*id as usize] =
                    value.perform_operation(value2, &registers, i64::wrapping_mul)
            }
            Token::Div(id, value, value2) => {
                registers[*id as usize] =
                    value.perform_operation(value2, &registers, i64::wrapping_div)
            }
            Token::Mod(id, value, value2) => {
                registers[*id as usize] =
                    value.perform_operation(value2, &registers, i64::wrapping_rem)
            }
            Token::Label(_) => {}
            Token::Jmp(Label::Base(label)) => index = *labels.get(label).unwrap(),
            Token::Jmp(Label::Address(address)) => index = *address,
        }
        index += 1;
    }
}
