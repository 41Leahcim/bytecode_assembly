use std::str::Chars;

/// An iterator over the characters in a piece of code
pub struct Code<'a> {
    line: usize,
    column: usize,
    buffer: Chars<'a>,
    last: Option<char>,
}

// Gives iterator functionality to Code
impl<'a> Iterator for Code<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // Get the current character
        let result = self.buffer.next();

        // If it's a new-line character, add 1 to the line number and set column to 0
        // Otherwise, move to the next column
        if result == Some('\n') {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        self.last = result;

        // Increment the index and return the character
        result
    }
}

impl<'a> Code<'a> {
    pub fn from_str(s: &'a str) -> Self {
        // Create the Code iterator, starting on column 0 of line 1
        Self {
            line: 1,
            column: 0,
            buffer: s.chars(),
            last: None,
        }
    }

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
        self.last.is_none()
    }
}
