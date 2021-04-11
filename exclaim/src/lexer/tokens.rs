use crate::common::Location;
use crate::common::serialize::*;


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

    pub fn lexeme(&self) -> &str {
        &self.lexeme
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}

impl Serializable for Token {
    fn serialize(&self, serde: &mut Serializer) {
        fn token_internals(token: &Token, serde: &mut Serializer) {
            Serializer::terminal(
                serde, 
                "kind", 
                || format!("{:?}", token.kind)
            );
            Serializer::terminal(
                serde,
                "lexeme",
                || format!("{:?}", token.lexeme)
            );
            token.location.serialize(serde);
        }

        Serializer::tag(
            serde, 
            "Token",
            |serde| token_internals(self, serde)
        );
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    StringLiteral,
    NumberLiteral(usize),

    Label,
    Operator(Op),
    Action(Action),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action {
    End,
    Let, 
    Write, 
    Render,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    And,            // &&
    Assign,         // =
    BlockClose,     // }}
    BlockClosePrime,// } Reserved
    BlockOpen,      // {{
    BlockOpenPrime, // { Reserved
    ClosureOpen,    // [ Reserved
    ClosureClose,   // ] Reserved
    Comma,          // , 
    Dot,            // . 
    Each,           // :
    Equality,       // ==
    Inequality,     // !=
    Or,             // || 
    Pipe,           // | (Chain function operations)
}



