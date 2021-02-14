use crate::util::Location;

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    lexeme: String, 
    location: Location,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, location: Location) -> Token {
        Token {
            kind,
            lexeme,
            location,
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    StringLiteral,
    NumberLiteral(usize),

    Label,
    Operator(Op),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Action,         // !
    Assign,         // =
    CloseBlock,     // }}
    CloseBlockPrime,// }
    Comma,          // , Keeping here if its used in future 
    Dot,            // . (Access fields)
    OpenBlock,      // {{
    OpenBlockPrime, // {
    Pipe,           // | (Chain function operations)
}



