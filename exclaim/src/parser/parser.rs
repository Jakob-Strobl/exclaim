use std::convert;

use crate::tokens;
use super::ast::AST;

pub struct Parser {
    tokenstream: Vec<tokens::Token>,
}

impl Parser {
    pub fn parse(self) {
        // Unimplemented
    }
}

impl convert::From<Vec<tokens::Token>> for Parser {
    fn from(tokens: Vec<tokens::Token>) -> Parser {
        Parser {
            tokenstream: tokens,
        }
    }
}

