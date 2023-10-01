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

/// Skips all whitespace, but errors on new lines
fn skip_whitespace(code: &mut Code) -> Result<Option<char>, Error> {
    let mut last_char = code.next();
    while last_char.is_some_and(char::is_whitespace) {
        if last_char == Some('\n') {
            return Err(Error::end_of_line(code.line(), code.column()));
        }
        last_char = code.next();
    }
    Ok(last_char)
}

/// Reads output
fn read_out(code: &mut Code) -> Result<Token, Error> {
    // Skip all whitespace
    // Error, if the end of the file was reached
    let Some(c) = skip_whitespace(code)? else {
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

/// Reads until the first non-whitespace character.
/// Returns the last read character, which is either non-whitespace or the last char of the code.
fn read_until_whitespace(code: &mut Code, mut last_char: char) -> (String, char) {
    let mut argument = last_char.to_string();
    for c in code.by_ref() {
        if c == ',' || c.is_whitespace() {
            last_char = c;
            break;
        }
        argument.push(c);
        last_char = c;
    }
    (argument, last_char)
}

/// Reads the first argument of most operations
fn read_first_argument(code: &mut Code) -> Result<(Value, char), Error> {
    // Skip the whitespace
    let Some(c) = skip_whitespace(code)? else {
        return Err(Error::end_of_file(code.line(), code.column()));
    };

    // Read the argument
    let (argument, last_char) = read_until_whitespace(code, c);

    // Convert the argument to a value and return it with the last read char
    Ok((Value::from_str(&argument, code), last_char))
}

/// Reads later arguments
fn read_later_argument(code: &mut Code, c: char) -> Result<(Value, char), Error> {
    // Look for the first seperator
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

    // Skip all whitespace
    let Some(last_char) = skip_whitespace(code)? else {
        return Err(Error::end_of_file(code.line(), code.column()));
    };

    // Read until whitespace or a seperator is found
    let (value, last_char) = read_until_whitespace(code, last_char);

    // Convert the string to a value and return it with the last read char
    Ok((Value::from_str(&value, code), last_char))
}

/// Reads multiple arguments, you should atleast take 1 argument.
/// Where SIZE is the number of arguments.
fn read_arguments<const SIZE: usize>(code: &mut Code) -> Result<[Value; SIZE], Error> {
    let mut arguments = [Value::Number(0); SIZE];
    assert!(
        SIZE > 1,
        "Not enough arguments, minimum = 1, {SIZE} requested"
    );
    let (value, mut ch) = read_first_argument(code)?;
    arguments[0] = value;
    for arg in arguments.iter_mut().skip(1) {
        let (value, c) = read_later_argument(code, ch)?;
        *arg = value;
        ch = c;
    }
    Ok(arguments)
}

/// Reads the arguments of the move operation and returns the operation with arguments
fn mov(code: &mut Code) -> Result<Token, Error> {
    // Read the arguments
    let arguments = read_arguments::<2>(code)?;

    // Make sure the first argument is a register, as it's only possible to move data into registers
    let Value::Register(register) = arguments[0] else {
        panic!(
            "You can only move into registers: {}:{}",
            code.line(),
            code.column()
        );
    };

    // Return the instruction
    Ok(Token::Mov(register, arguments[1]))
}

/// Reads the add operation, returns the add operation with arguments
fn add(code: &mut Code) -> Result<Token, Error> {
    // Read the arguments
    let arguments = read_arguments::<3>(code)?;

    // Make sure the first argument is a register
    let Value::Register(register) = arguments[0] else {
        panic!(
            "You can only move into registers: {}:{}",
            code.line(),
            code.column()
        );
    };

    // Return the add operation
    Ok(Token::Add(register, arguments[1], arguments[2]))
}

/// Parses the passed command
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
