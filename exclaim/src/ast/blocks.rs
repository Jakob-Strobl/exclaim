// Temporary type names 
use crate::common::serialize::*;
use crate::tokens::Token;

use super::AstIndex;

// Hopefully, helps clarifies what types of data structures the index points to
type BlockIndex = AstIndex;
type StatementIndex = AstIndex;
type Scope = Vec<AstIndex>;

pub enum Block {
    /// Text(text: Token, next_block: Option<AstIndex>)
    Text(Token, Option<BlockIndex>),
    /// CodeEnclosed(stmt: AstIndex, next_block: Option<AstIndex>)
    CodeEnclosed(StatementIndex, Option<BlockIndex>),
    /// CodeUnclosed(stmt: AstIndex, scope: Vec<AstIndex>, next_block: Option<AstIndex>)
    CodeUnclosed(StatementIndex, Scope, Option<BlockIndex>),
    /// CodeClosing(stmt: AstIndex, next_block: Option<AstIndex>)
    CodeClosing(StatementIndex, Option<BlockIndex>),
}

impl Block {
    pub fn text(&self) -> Option<&Token> {
        match self {
            Block::Text(text, _) => Some(text),
            _ => None,
        }
    }

    pub fn stmt(&self) -> Option<&StatementIndex> {
        match self {
            Block::CodeEnclosed(stmt, _) => Some(stmt),
            Block::CodeUnclosed(stmt, _, _) => Some(stmt),
            Block::CodeClosing(stmt, _) => Some(stmt),
            _ => None,
        }
    }

    pub fn set_stmt(&mut self, statement: StatementIndex) {
        match self {
            Block::Text(_, _) => {}
            Block::CodeEnclosed(stmt, _) => *stmt = statement,
            Block::CodeUnclosed(stmt, _, _) => *stmt = statement,
            Block::CodeClosing(stmt, _) => *stmt = statement,
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
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        match self {
            Block::Text(text, next) => {
                let _block = serde.open_tag("TextBlock");
                text.serialize(serde, ctx);
                *next // copy
            }
            Block::CodeEnclosed(statement, next) => {
                let _block = serde.open_tag("EnclosedBlock");
                statement.serialize(serde, ctx);
                *next // copy
            }
            Block::CodeUnclosed(statement, scope, next) => {
                let _block = serde.open_tag("UnclosedBlock");
                statement.serialize(serde, ctx);
                scope.serialize(serde, ctx);
                *next // copy
            }
            Block::CodeClosing(statement, next) => {
                let _block = serde.open_tag("ClosingBlock");
                statement.serialize(serde, ctx);
                *next // copy
            }
        }
    }
}