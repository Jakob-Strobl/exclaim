// Temporary type names 
use crate::common::serialize::*;
use crate::tokens::Token;

use super::AstIndex;
use super::statements::Statement;

type Scope = Vec<Block>;

pub enum Block {
    Text(Token, Option<AstIndex>),
    CodeEnclosed(Statement, Option<AstIndex>),
    CodeUnclosed(Statement, Scope, Option<AstIndex>),
    CodeClosing(Statement, Option<AstIndex>),
}

impl Block {
    pub fn text(&self) -> Option<&Token> {
        match self {
            Block::Text(text, _) => Some(text),
            _ => None,
        }
    }

    pub fn next(&self) -> &Option<AstIndex> {
        match self {
            Block::Text(_, index) => index,
            Block::CodeEnclosed(_, index) => index,
            Block::CodeUnclosed(_, _, index) => index,
            Block::CodeClosing(_, index) => index,
        }
    }

    pub fn set_next(&mut self, index: AstIndex) {
        match self {
            Block::Text(_, idx) => *idx = Some(index),
            Block::CodeEnclosed(_, idx) => *idx = Some(index),
            Block::CodeUnclosed(_, _, idx) => *idx = Some(index),
            Block::CodeClosing(_, idx) => *idx = Some(index),
        }
    }
}

impl Serializable for Block {
    fn serialize(&self, serde: &mut Serializer) -> &Option<AstIndex> {
        match self {
            Block::Text(text, next) => {
                let _block = serde.open_tag("TextBlock");
                text.serialize(serde);
                next
            }
            Block::CodeEnclosed(stmt, next) => {
                let _block = serde.open_tag("EnclosedBlock");
                stmt.serialize(serde);
                next
            }
            Block::CodeUnclosed(stmt, _, next) => {
                let _block = serde.open_tag("UnclosedBlock");
                stmt.serialize(serde);
                next
            }
            Block::CodeClosing(stmt, next) => {
                let _block = serde.open_tag("ClosingBlock");
                stmt.serialize(serde);
                next
            }
        }
    }
}