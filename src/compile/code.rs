use std::str::FromStr;

/// An iterator over the characters in a piece of code
pub struct Code {
    line: usize,
    column: usize,
    index: usize,
    buffer: Vec<char>,
}

// Creates a new code iterator from a string slice
impl FromStr for Code {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Collect the characters in a vector to make access to chars easier
        let buffer = s.chars().collect::<Vec<char>>();

        // Create the Code iterator, starting on column 0 of line 1
        Ok(Self {
            line: 1,
            column: 0,
            buffer,
            index: 0,
        })
    }
}

// Gives iterator functionality to Code
impl Iterator for Code {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the current character
        let result = self.buffer.get(self.index).map(char::to_owned);

        // If it's a new-line character, add 1 to the line number and set column to 0
        // Otherwise, move to the next column
        if result == Some('\n') {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }

        // Increment the index and return the character
        self.index += 1;
        result
    }
}

// Allows to take a specific character
impl std::ops::Index<usize> for Code {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.get(index).unwrap()
    }
}

impl Code {
    /// Returns the line number
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Returns the column number
    pub const fn column(&self) -> usize {
        self.column
    }

    /// Returns whether the end of the file was reached
    pub fn eof(&self) -> bool {
        self.index > self.buffer.len()
    }

    /// Returns the length of the buffer
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the number of bytes read
    pub const fn bytes_read(&self) -> usize {
        self.index
    }
}
