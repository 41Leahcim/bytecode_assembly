use super::Token;
use code::Code;
use std::str::FromStr;

mod code;

fn read_comment(code: &mut Code) -> Token {
    let mut comment = String::new();
    let Some(mut last_char) = code.next() else {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    };
    for c in code.by_ref() {
        if last_char == '*' && c == '/' {
            break;
        } else if last_char != '*' {
            comment.push(last_char);
        }
        last_char = c;
    }
    if code.eof() {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    }
    Token::Comment(comment)
}

pub fn read_string(code: &mut Code) -> String {
    let mut result = String::new();
    let mut escaped = false;
    for c in code.by_ref() {
        if escaped {
            result.push(match c {
                'n' => '\n',
                'r' => '\r',
                '\\' | '"' => c,
                't' => '\t',
                '0' => '\0',
                _ => panic!(
                    "Invalid escape character '{c}': {}:{}",
                    code.line(),
                    code.column()
                ),
            });
            escaped = false;
        } else if c == '"' {
            break;
        } else if c == '\\' {
            escaped = true;
        } else {
            result.push(c);
        }
    }
    result
}

pub fn read_out(code: &mut Code) -> Token {
    let mut c = code.next();
    while let Some(ch) = c {
        if !ch.is_whitespace() {
            break;
        }
        c = code.next();
    }
    let Some(c) = c else {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    };
    if c == '"' {
        return Token::Out(read_string(code));
    }
    let mut output = c.to_string();
    for c in code {
        if c.is_whitespace() {
            break;
        }
        output.push(c);
    }
    Token::Out(output)
}

pub fn split_tokens(code: &str) -> Vec<Token> {
    let mut code = Code::from_str(code).unwrap();
    let mut tokens = Vec::new();
    let Some(mut last_char) = code.next() else {
        return vec![];
    };
    let mut command = String::new();
    while let Some(c) = code.next() {
        if c.is_whitespace() {
            match command.as_str() {
                "" => {}
                "out" => tokens.push(read_out(&mut code)),
                _ => todo!(),
            }
            command.clear();
        } else if last_char == '/' && c == '*' {
            tokens.push(read_comment(&mut code));
            command.clear();
        } else {
            command.push(c);
        }
        last_char = c;
    }
    if !command.is_empty() {
        match command.as_str() {
            "" => {}
            "out" => tokens.push(read_out(&mut code)),
            _ => panic!(
                "Invalid command \"{command}\" at: {}:{}",
                code.line(),
                code.column()
            ),
        }
    }
    tokens
}
