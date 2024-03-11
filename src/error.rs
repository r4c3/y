use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum InterpreterError {
    LexerError(LexerError),
    ParserError(ParserError),
    RuntimeError(RuntimeError),
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter { line: usize, character: char },
    UnterminatedString { line: usize },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter { line, character } => {
                write!(f, "Unexpected character '{}' at line {}", character, line)
            }
            LexerError::UnterminatedString { line } => {
                write!(f, "Unterminated string at line {}", line)
            }
        }
    }
}

impl Error for LexerError {}

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub line: usize,
}

impl ParserError {
    pub fn new(message: String, line: usize) -> Self {
        Self { message, line }
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}

impl RuntimeError {
    pub fn new(message: String, line: usize) -> Self {
        Self { message, line }
    }
}
