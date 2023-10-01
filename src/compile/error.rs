use std::fmt::Debug;

#[derive(Clone, Copy)]
enum ErrorKind {
    EndOfLine,
    EndOfFile,
}

#[derive(Clone, Copy)]
pub struct Error {
    kind: ErrorKind,
    line: usize,
    column: usize,
}

impl Error {
    pub const fn end_of_line(line: usize, column: usize) -> Self {
        Self {
            kind: ErrorKind::EndOfLine,
            line,
            column,
        }
    }

    pub const fn end_of_file(line: usize, column: usize) -> Self {
        Self {
            kind: ErrorKind::EndOfFile,
            line,
            column,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::EndOfFile => {
                write!(f, "Unexpected End Of File: {}:{}", self.line, self.column)
            }
            ErrorKind::EndOfLine => {
                write!(f, "Unexpected End Of Line: {}:{}", self.line, self.column)
            }
        }
    }
}
