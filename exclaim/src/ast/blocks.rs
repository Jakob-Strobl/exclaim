// Temporary type names 
use crate::tokens::Token;
use super::AstIndex;

type Statement = ();
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