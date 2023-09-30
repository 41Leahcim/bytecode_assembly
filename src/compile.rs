use crate::value::Value;

use super::Token;
use code::Code;
use std::str::FromStr;

mod code;

/// Reads a multi-line comment
fn read_comment(code: &mut Code) -> Token {
    // Create a buffer for the string content
    let mut comment = String::new();

    // Read the first char of the string, error if EOF is reached
    let Some(mut last_char) = code.next() else {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    };

    // Iterate over the chars in the comment
    for c in code.by_ref() {
        // Stop reading if the end of the comment was reached
        if last_char == '*' && c == '/' {
            break;
        } else {
            comment.push(last_char);
        }

        // Store the current char as last char
        last_char = c;
    }

    // Error, if the end of the code was reached before the comment was closed
    if code.eof() {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    }

    // Return the read comment
    Token::Comment(comment)
}

/// Reads a string
fn read_string(code: &mut Code) -> String {
    // Create a buffer for the string and a variable to keep track of escaped chars
    let mut result = String::new();
    let mut escaped = false;

    // Iterate over the chars
    for c in code.by_ref() {
        if escaped {
            // If the current char is escaped, try converting it to the correct character
            // Error, if it is an invalid escape character
            // Push on success
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

            // Set escaped to false to not escape more characters than needed
            escaped = false;
        } else if c == '"' {
            // Break if the current char is a double quote
            break;
        } else if c == '\\' {
            // If the current character is a '\', the next is escaped.
            escaped = true;
        } else {
            // Push the current character to the string in all other cases
            result.push(c);
        }
    }

    // Error, if the end of the code is reached before finding a double quote
    if code.eof() {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    }
    result
}

fn skip_whitespace(code: &mut Code) -> Option<char> {
    let mut c = code.next();
    while let Some(ch) = c {
        if !ch.is_whitespace() {
            break;
        }
        c = code.next();
    }
    c
}

/// Reads output
fn read_out(code: &mut Code) -> Token {
    // Skip all whitespace
    // Error, if the end of the file was reached
    let Some(c) = skip_whitespace(code) else {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    };

    // If the current char is a double quote, the value is a string
    // Read the string and return it in an output token
    if c == '"' {
        return Token::Out(read_string(code));
    }

    // Convert the current char to a string
    let mut output = c.to_string();

    // Add all chars until the next whitespace
    for c in code {
        if c.is_whitespace() {
            break;
        }
        output.push(c);
    }

    // Add a new line and return an output token with the resulting string
    output.push('\n');
    Token::Out(output)
}

pub fn mov(code: &mut Code) -> Token {
    // Skip the whitespace
    let Some(c) = skip_whitespace(code) else {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    };

    if c != 'r' {
        panic!(
            "You can only move into registers: {}:{}",
            code.line(),
            code.column()
        );
    }

    let mut id = String::new();
    let mut seperator_found = false;
    for c in code.by_ref() {
        if c == ',' || c.is_whitespace() {
            seperator_found = c == ',';
            break;
        }
        id.push(c);
    }
    let Ok(id) = id.parse::<u8>() else {
        panic!(
            "Invalid register id \"{id}\": {}:{}",
            code.line(),
            code.column()
        );
    };
    while !seperator_found {
        match code.next() {
            None => panic!("Unexpected End Of File: {}:{}", code.line(), code.column()),
            Some(',') => seperator_found = true,
            Some('\n') => panic!("Unexpected End Of Line: {}:{}", code.line(), code.column()),
            Some(_) => {}
        }
    }
    let mut last_char = code.next();
    while last_char.is_some_and(|c| c.is_whitespace()) {
        if last_char == Some('\n') {
            panic!("Unexpected End Of Line: {}:{}", code.line(), code.column());
        }
        last_char = code.next();
    }
    let Some(last_char) = last_char else {
        panic!("Unexpected End Of File: {}:{}", code.line(), code.column());
    };
    if last_char == '\n' {
        panic!("Unexpected End Of Line: {}:{}", code.line(), code.column());
    }
    let value =
        code.take_while(|c| !c.is_whitespace())
            .fold(last_char.to_string(), |mut out, c| {
                out.push(c);
                out
            });
    let value = value.trim();
    let Ok(value) = value.parse::<i64>() else {
        panic!(
            "Invalid value \"{value}\": {}:{}",
            code.line(),
            code.column()
        );
    };
    Token::Mov(id, Value::Number(value))
}

pub fn parse_command(command: &str, code: &mut Code) -> Option<Token> {
    match command {
        "" => None,
        "out" => Some(read_out(code)),
        "mov" => Some(mov(code)),
        _ => panic!(
            "Invalid command \"{command}\" at: {}:{}",
            code.line(),
            code.column()
        ),
    }
}

/// Splits the code into tokens
pub fn split_tokens(code: &str) -> Vec<Token> {
    // Create a new code iterator and a vector for the tokens
    let mut code = Code::from_str(code).unwrap();
    let mut tokens = Vec::new();

    // If the code is empty, return the empty vector
    let Some(mut last_char) = code.next() else {
        return tokens;
    };

    // Create a command string
    let mut command = if last_char.is_whitespace() {
        String::new()
    } else {
        last_char.to_string()
    };

    // Iterate through the code
    while let Some(c) = code.next() {
        if c.is_whitespace() {
            // If the current char is whitespace
            // Try to parse the current command
            if let Some(token) = parse_command(&command, &mut code) {
                tokens.push(token);
            };

            // Clear the command
            command.clear();
        } else if last_char == '/' && c == '*' {
            // Else if the last and current char form the start of a comment
            // Try to push the comment to the token vector
            tokens.push(read_comment(&mut code));

            // Clear the command
            command.clear();
        } else {
            // Push the current char to the command
            command.push(c);
        }
        // Set the current char as last char
        last_char = c;
    }
    // Try to parse the current command if the command isn't empty
    if let Some(token) = parse_command(&command, &mut code) {
        tokens.push(token);
    }
    tokens
}
