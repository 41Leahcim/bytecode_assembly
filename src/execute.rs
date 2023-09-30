use crate::{value::Value, Token};
use std::{
    fmt::Write as _,
    io::{self, Write as _},
};

pub fn out(output: &str, registers: &[Value]) {
    let mut result = String::new();
    let mut output = output.chars();
    let mut register = String::new();
    while let Some(c) = output.next() {
        if c == '{' {
            let mut last_char = '{';
            for c in output.by_ref() {
                if c == '}' || c == '{' {
                    last_char = c;
                    break;
                }
                register.push(c);
                last_char = c;
            }
            if last_char == '{' {
                result.push('{');
            } else {
                match register.parse::<u8>() {
                    Ok(index) => write!(&mut result, "{}", registers[index as usize]).unwrap(),
                    Err(_) => write!(&mut result, "{{{register}").unwrap(),
                }
            }
            register.clear();
        } else {
            result.push(c);
        }
    }
    io::stdout().lock().write_all(result.as_bytes()).unwrap();
}

pub fn execute(tokens: &[Token]) {
    let mut registers = [Value::Number(0); 256];
    // Iterate through the tokens
    for token in tokens {
        // Execute the current token
        match token {
            Token::Comment(_) => {}
            Token::Out(output) => out(output, &registers),
            Token::Mov(id, value) => registers[*id as usize] = *value,
        }
    }
}
