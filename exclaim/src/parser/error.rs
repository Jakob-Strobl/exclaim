// This code's structure heavily inspired by Rust's std::io::error.rs
use std::fmt::{self, Pointer};

use crate::tokens::Token;

pub struct ParserError {
    error: Error,
}

impl fmt::Debug for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.error, f)
    }
}

impl From<ErrorKind> for ParserError {
    fn from(kind: ErrorKind) -> ParserError {
        ParserError {
            error: Error::Simple(kind),
        }
    }
}

impl From<&str> for ParserError {
    fn from(msg: &str) -> ParserError {
        ParserError {
            error: Error::Custom(msg.to_string()),
        }
    }
}

impl From<String> for ParserError {
    fn from(msg: String) -> ParserError {
        ParserError {
            error: Error::Custom(msg),
        }
    }
}

enum Error {
    Simple(ErrorKind),
    Custom(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Simple(kind) => f.debug_tuple("Kind").field(&kind).finish(),
            Error::Custom(msg) => f.debug_tuple("Custom").field(&msg).finish(),
        }
    }
}

pub enum ErrorKind {
    // Reached a section of the parser that hasn't been implemented
    Unimplemented,
    // Found String Literal where it wasnt supposed to be
    UnexpectedStringLiteral,
    // End of Token Stream, but we expected another token 
    UnexpectedEndOfTokenStream,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::Unimplemented => "reached unimplemented code",
            ErrorKind::UnexpectedStringLiteral => "unexpected string literal",
            ErrorKind::UnexpectedEndOfTokenStream => "unexpected end of token stream",
        }
    }
}

impl fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.as_str(), f)
    }
}