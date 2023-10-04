use crate::token::{
    argument::{read_arguments, skip_whitespace},
    Label,
};

use super::Token;
use code::Code;
use error::Error;
use std::{collections::HashMap, str::FromStr};

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

/// Reads labels for branching
fn read_jmp_label(code: &mut Code) -> Result<String, Error> {
    // Skip all whitespace
    let Some(c) = skip_whitespace(code)? else {
        return Err(Error::end_of_file(code.line(), code.column()));
    };

    // Read the label
    let label = code
        .take_while(|c| !c.is_whitespace())
        .fold(c.to_string(), |mut label, c| {
            label.push(c);
            label
        });
    Ok(label)
}

/// Parses the passed command
fn parse_command(command: &str, code: &mut Code) -> Result<Option<Token>, Error> {
    match command {
        "" => Ok(None),
        "out" => Ok(Some(read_out(code)?)),
        "mov" => Ok(Some(Token::mov(code)?)),
        "add" => Ok(Some(Token::add(code)?)),
        "sub" => Ok(Some(Token::sub(code)?)),
        "mul" => Ok(Some(Token::mul(code)?)),
        "div" => Ok(Some(Token::div(code)?)),
        "mod" => Ok(Some(Token::modulo(code)?)),
        "jmp" => Ok(Some(Token::Jmp(Label::Base(read_jmp_label(code)?)))),
        "jl" => Ok(Some(Token::Jl(Label::Base(read_jmp_label(code)?)))),
        "jg" => Ok(Some(Token::Jg(Label::Base(read_jmp_label(code)?)))),
        "je" => Ok(Some(Token::Je(Label::Base(read_jmp_label(code)?)))),
        "cmp" => {
            let args = read_arguments::<2>(code)?;
            Ok(Some(Token::Cmp(args[0], args[1])))
        }
        label if label.ends_with(':') => {
            let label_name = label.chars().take_while(|c| *c != ':').collect::<String>();
            if label.len() != label_name.len() + 1 {
                panic!(
                    "Invalid label \"{command}\" at: {}:{}",
                    code.line(),
                    code.column()
                );
            }
            Ok(Some(Token::Label(label_name)))
        }
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

fn convert_label(label: &Label, labels: &HashMap<String, usize>) -> Label {
    match label {
        Label::Base(label) => Label::Address(
            *labels
                .get(label)
                .unwrap_or_else(|| panic!("Label \"{label}\" doesn't exist")),
        ),
        Label::Address(address) => Label::Address(*address),
    }
}

pub fn convert_labels(tokens: &mut [Token]) {
    let labels = read_labels(tokens);
    for token in tokens {
        #[allow(clippy::single_match)]
        match token {
            Token::Jmp(label) => *token = Token::Jmp(convert_label(label, &labels)),
            Token::Jl(label) => *token = Token::Jl(convert_label(label, &labels)),
            Token::Je(label) => *token = Token::Je(convert_label(label, &labels)),
            Token::Jg(label) => *token = Token::Jg(convert_label(label, &labels)),
            _ => {}
        }
    }
}
