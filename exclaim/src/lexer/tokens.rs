use crate::util::Location;
use crate::Serializeable;
use crate::AstSerializer;


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

impl Serializeable for Token {
    fn serialize(&self, serde: &mut crate::AstSerializer) {
        fn token_internals(token: &Token, serde: &mut AstSerializer) {
            AstSerializer::terminal(
                serde, 
                "kind", 
                || format!("{:?}", token.kind)
            );
            AstSerializer::terminal(
                serde,
                "lexeme",
                || format!("{:?}", token.lexeme)
            );
            token.location.serialize(serde);
        }

        AstSerializer::tag(
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



