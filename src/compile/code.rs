use std::str::FromStr;

pub struct Code {
    line: usize,
    column: usize,
    index: usize,
    buffer: Vec<char>,
}

impl FromStr for Code {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buffer = s.chars().collect::<Vec<char>>();
        Ok(Self {
            line: 1,
            column: 0,
            buffer,
            index: 0,
        })
    }
}

impl Iterator for Code {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.buffer.get(self.index).map(char::to_owned);
        if result == Some('\n') {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        self.index += 1;
        result
    }
}

impl std::ops::Index<usize> for Code {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.get(index).unwrap()
    }
}

impl Code {
    pub const fn line(&self) -> usize {
        self.line
    }

    pub const fn column(&self) -> usize {
        self.column
    }

    pub fn eof(&self) -> bool {
        self.index > self.buffer.len()
    }
}
