use crate::value::Value;

use super::Token;
use code::Code;
use error::Error;
use std::str::FromStr;

pub mod code;
pub mod error;

/// Reads a multi-line comment
fn read_comment(code: &mut Code) -> Result<Token, Error> {
    // Create a buffer for the string content
    let mut comment = String::new();

    // Read the first char of the string, error if EOF is reached
    let Some(mut last_char) = code.next() else {
        return Err(Error::end_of_file(code.line(), code.column()));
    };

    // Iterate over the chars in the comment
    for c in code.by_ref() {
        // Stop reading if the end of the comment was reached
        if last_char == '*' && c == '/' {
            break;
        }
        comment.push(last_char);

        // Store the current char as last char
        last_char = c;
    }

    // Error, if the end of the code was reached before the comment was closed
    if code.eof() {
        return Err(Error::end_of_file(code.line(), code.column()));
    }

    // Return the read comment
    Ok(Token::Comment(comment))
}

/// Reads a string
fn read_string(code: &mut Code) -> Result<String, Error> {
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
        Err(Error::end_of_file(code.line(), code.column()))
    } else {
        Ok(result)
    }
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
fn read_out(code: &mut Code) -> Result<Token, Error> {
    // Skip all whitespace
    // Error, if the end of the file was reached
    let Some(c) = skip_whitespace(code) else {
        return Err(Error::end_of_file(code.line(), code.column()));
    };

    // If the current char is a double quote, the value is a string
    // Read the string and return it in an output token
    if c == '"' {
        return Ok(Token::Out(read_string(code)?));
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
    Ok(Token::Out(output))
}

fn read_first_argument(code: &mut Code) -> Result<(Value, char), Error> {
    // Skip the whitespace
    let Some(c) = skip_whitespace(code) else {
        return Err(Error::end_of_file(code.line(), code.column()));
    };

    let mut argument = c.to_string();
    let mut last_char = c;
    for c in code.by_ref() {
        if c == ',' || c.is_whitespace() {
            last_char = c;
            break;
        }
        argument.push(c);
        last_char = c;
    }

    Ok((Value::from_str(&argument, code), last_char))
}

fn read_later_argument(code: &mut Code, c: char) -> Result<(Value, char), Error> {
    let mut seperator_found = c == ',';
    while !seperator_found {
        match code.next() {
            None => return Err(Error::end_of_file(code.line(), code.column())),
            Some(',') => seperator_found = true,
            Some('\n') => return Err(Error::end_of_line(code.line(), code.column())),
            Some(c) if c.is_whitespace() => {}
            Some(c) => panic!(
                "Unexpected character \"{c}\": {}:{}",
                code.line(),
                code.column()
            ),
        }
    }
    let mut last_char = code.next();
    while last_char.is_some_and(char::is_whitespace) {
        if last_char == Some('\n') {
            return Err(Error::end_of_line(code.line(), code.column()));
        }
        last_char = code.next();
    }
    let Some(mut last_char) = last_char else {
        return Err(Error::end_of_file(code.line(), code.column()));
    };
    if last_char == '\n' {
        return Err(Error::end_of_line(code.line(), code.column()));
    }
    let value = code.take_while(|c| !c.is_whitespace() && *c != ',').fold(
        last_char.to_string(),
        |mut out, c| {
            last_char = c;
            out.push(c);
            out
        },
    );
    let value = value.trim();
    Ok((Value::from_str(value, code), c))
}

fn mov(code: &mut Code) -> Result<Token, Error> {
    let (value, c) = read_first_argument(code)?;

    let Value::Register(register) = value else {
        panic!(
            "You can only move into registers: {}:{}",
            code.line(),
            code.column()
        );
    };

    let (value, _) = read_later_argument(code, c)?;
    Ok(Token::Mov(register, value))
}

fn add(code: &mut Code) -> Result<Token, Error> {
    let (value, c) = read_first_argument(code)?;

    let Value::Register(register) = value else {
        panic!(
            "You can only add into registers: {}:{}",
            code.line(),
            code.column()
        );
    };

    let (value, c) = read_later_argument(code, c)?;
    let (value2, _) = read_later_argument(code, c)?;
    Ok(Token::Add(register, value, value2))
}

fn parse_command(command: &str, code: &mut Code) -> Result<Option<Token>, Error> {
    match command {
        "" => Ok(None),
        "out" => Ok(Some(read_out(code)?)),
        "mov" => Ok(Some(mov(code)?)),
        "add" => Ok(Some(add(code)?)),
        _ => panic!(
            "Invalid command \"{command}\" at: {}:{}",
            code.line(),
            code.column()
        ),
    }
}

/// Splits the code into tokens
pub fn split_tokens(code: &str) -> Result<Vec<Token>, Error> {
    // Create a new code iterator and a vector for the tokens
    let mut code = Code::from_str(code).unwrap();
    let mut tokens = Vec::new();

    // If the code is empty, return the empty vector
    let Some(mut last_char) = code.next() else {
        return Ok(tokens);
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
            if let Some(token) = parse_command(&command, &mut code)? {
                tokens.push(token);
            };

            // Clear the command
            command.clear();
        } else if last_char == '/' && c == '*' {
            // Else if the last and current char form the start of a comment
            // Try to push the comment to the token vector
            tokens.push(read_comment(&mut code)?);

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
    if let Some(token) = parse_command(&command, &mut code)? {
        tokens.push(token);
    }
    Ok(tokens)
}
