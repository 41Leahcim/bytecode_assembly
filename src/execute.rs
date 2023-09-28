use crate::Token;

pub fn execute(tokens: &[Token]) {
    for token in tokens {
        match token {
            Token::Comment(_) => {}
            Token::Out(output) => print!("{output}"),
        }
    }
}
