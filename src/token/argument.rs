use crate::{
    compile::{code::Code, error::Error},
    value::Value,
};

/// Skips all whitespace, but errors on new lines
pub fn skip_whitespace(code: &mut Code) -> Result<Option<char>, Error> {
    let mut last_char = code.next();
    while last_char.is_some_and(char::is_whitespace) {
        if last_char == Some('\n') {
            return Err(Error::end_of_line(code.line(), code.column()));
        }
        last_char = code.next();
    }
    Ok(last_char)
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

/// Reads one register and multiple arguments.
/// Where SIZE is the number of arguments.
pub fn read_arguments<const SIZE: usize>(code: &mut Code) -> Result<(u8, [Value; SIZE]), Error> {
    let mut arguments = [Value::Number(0); SIZE];
    let (value, mut ch) = read_first_argument(code)?;

    // Make sure the first argument is a register
    let Value::Register(register) = value else {
        panic!(
            "You can only move into registers: {}:{}",
            code.line(),
            code.column()
        );
    };

    // Read arguments
    for arg in arguments.iter_mut() {
        let (value, c) = read_later_argument(code, ch)?;
        *arg = value;
        ch = c;
    }
    Ok((register, arguments))
}
