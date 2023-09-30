use crate::{value::Value, Token};

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
                    Ok(index) => result += &registers[index as usize].to_string(),
                    Err(_) => {
                        result.push('{');
                        result += &register;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }
    print!("{result}");
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
