use crate::{value::Value, Token};

pub fn execute(tokens: &[Token]) {
    let mut registers = [Value::Number(0); 256];
    // Iterate through the tokens
    for token in tokens {
        // Execute the current token
        match token {
            Token::Comment(_) => {}
            Token::Out(output) => print!("{output}"),
            Token::Mov(id, value) => registers[*id as usize] = *value,
        }
    }
}
