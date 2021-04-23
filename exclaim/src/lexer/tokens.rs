use crate::ast::AstIndex;
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
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        // TODO write explicitly per token-kind 
        let _token = serde.open_tag("Token");
        serde.terminal("kind", format!("{:?}", self.kind).as_str());
        serde.terminal("lexeme", format!("{:?}", self.lexeme).as_str());
        self.location.serialize(serde, ctx);
        None
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
    Render,
    Write,
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
    ParenOpen,      // (
    ParenClose,     // )
    Pipe,           // | (Chain function operations)
}



