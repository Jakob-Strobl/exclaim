use crate::ast::AstIndex;
use crate::common::Location;
use crate::common::serialize::*;

use crate::data::traits::Renderable;

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
        match self.kind {
            TokenKind::StringLiteral => {
                let _token = serde.open_tag("StringLiteral");
                serde.terminal("value", &format!("{:?}", self.lexeme()));
                self.location().serialize(serde, ctx)
            }
            TokenKind::NumberLiteral(num) => {
                let _token = serde.open_tag("NumberLiteral");
                serde.terminal("value", &num.to_string());
                self.location().serialize(serde, ctx)
            }
            TokenKind::Label => {
                let _token = serde.open_tag("Label");
                serde.terminal("value", &format!("{:?}", self.lexeme()));
                self.location().serialize(serde, ctx)
            }
            TokenKind::Operator(op) => {
                let _token = serde.open_tag("Operator");
                serde.terminal("value", &format!("{:?}", op));
                self.location().serialize(serde, ctx)
            }
            TokenKind::Action(action) => {
                let _token = serde.open_tag("Action");
                serde.terminal("value", &format!("{:?}", action));
                self.location().serialize(serde, ctx)
            }
        }
    }
}

impl Renderable for Token {
    fn render(&self) -> String {
        match self.kind() {
            TokenKind::StringLiteral => self.lexeme().to_string(),
            TokenKind::NumberLiteral(num) => num.to_string(),
            _ => panic!("Renderable Token panicked!"),
        }
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



