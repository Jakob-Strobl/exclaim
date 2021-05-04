use crate::ast::AstIndex;
use crate::common::Location;
use crate::common::serialize::*;

use crate::data::traits::Renderable;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    StringLiteral(String, Location),
    NumberLiteral(usize, Location),

    Label(String, Location),
    Operator(Op, Location),
    Action(Action, Location),
}

impl Token {
    pub fn label(&self) -> Option<&str> {
        match self {
            Token::Label(label, _) => Some(label),
            _ => None,
        }
    }
}

impl Serializable for Token {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        match self {
            Token::StringLiteral(literal, loc) => {
                let _token = serde.open_tag("StringLiteral");
                serde.terminal("value", &format!("{:?}", literal));
                loc.serialize(serde, ctx)
            }
            Token::NumberLiteral(literal, loc) => {
                let _token = serde.open_tag("NumberLiteral");
                serde.terminal("value", &literal.to_string());
                loc.serialize(serde, ctx)
            }
            Token::Label(label, loc) => {
                let _token = serde.open_tag("Label");
                serde.terminal("value", &format!("{:?}", label));
                loc.serialize(serde, ctx)
            }
            Token::Operator(op, loc) => {
                let _token = serde.open_tag("Operator");
                serde.terminal("value", &format!("{:?}", op));
                loc.serialize(serde, ctx)
            }
            Token::Action(action, loc) => {
                let _token = serde.open_tag("Action");
                serde.terminal("value", &format!("{:?}", action));
                loc.serialize(serde, ctx)
            }
        }
    }
}

impl Renderable for Token {
    fn render(&self) -> String {
        match self {
            Token::StringLiteral(literal, _) => literal.to_string(),
            Token::NumberLiteral(literal, _) => literal.to_string(),
            Token::Label(label, _) => label.to_string(),
            Token::Operator(op, _) => format!("{:?}", op),
            Token::Action(action, _) => format!("{:?}", action),
        }
    }
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



