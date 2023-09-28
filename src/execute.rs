use crate::Token;

pub fn execute(tokens: &[Token]) {
    // Iterate through the tokens
    for token in tokens {
        // Execute the current token
        match token {
            Token::Comment(_) => {}
            Token::Out(output) => print!("{output}"),
        }
    }
}
