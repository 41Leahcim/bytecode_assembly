use std::fmt::Debug;

/// The kind of compilation error
#[derive(Clone, Copy)]
enum ErrorKind {
    EndOfLine,
    EndOfFile,
}

/// The actual error
#[derive(Clone, Copy)]
pub struct Error {
    kind: ErrorKind,
    line: usize,
    column: usize,
}

impl Error {
    /// Creates an end of line error
    pub const fn end_of_line(line: usize, column: usize) -> Self {
        Self {
            kind: ErrorKind::EndOfLine,
            line,
            column,
        }
    }

    /// Creates an end of file error
    pub const fn end_of_file(line: usize, column: usize) -> Self {
        Self {
            kind: ErrorKind::EndOfFile,
            line,
            column,
        }
    }
}

/// The output on unwrap or when printed in debug mode
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
