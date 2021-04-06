use std::convert;
use std::ops::{Deref, DerefMut};
use std::result;
use std::collections::LinkedList;

use super::ast::Ast;
use super::node::*;
use super::error::{
    ParserError,
    ErrorKind,
};
use crate::tokens::{
    Token,
    TokenKind,
    Op
};

type Result<T> = result::Result<Box<T>, ParserError>;
pub struct ParserList<T>(LinkedList<T>);
impl<T> ParserList<T> {
    fn lookahead(&self) -> Option<&T> {
        self.0.iter().nth(1)
    }
}
impl<T> Deref for ParserList<T> {
    type Target = LinkedList<T>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for ParserList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Parser {
    token_stream: ParserList<Token>,
    token_index: usize,
    ast: Ast,
}

// Methods
impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.token_stream.front()
    }

    fn lookahead(&self) -> Option<&Token> {
        self.token_stream.lookahead()
    }

    fn consume(&mut self) -> Token {
        self.token_stream.pop_front().unwrap()
    }

    fn end_of_token_stream(&self) -> bool {
        self.token_stream.is_empty()
    }
}

// Associated functions -> Parsing functions
impl Parser {
    pub fn parse(parser: &mut Parser) -> result::Result<Ast, ParserError> {
        let mut ast = Ast::new();

        let result = Parser::p_start_block(parser);

        match result {
            Ok(node) => {
                ast.push_block(node);
                Ok(ast)
            },
            Err(e) => Err(e),
        }
    }

    fn p_start_block(parser: &mut Parser) -> Result<dyn Node> {
        if let Some(token) = parser.peek() {
            match token.kind() {
                &TokenKind::StringLiteral => {
                    Ok(Box::new(TextNode::new(parser.consume())))
                },
                _ => Err(ParserError::from(ErrorKind::Unimplemented))
            }
        } else {
            Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        }
    }

    // fn p_start_block(parser: Parser) -> Result<BlockNode> {
    //     let token = parser.peek();

    //     match token.kind() {
    //         TokenKind::Operator(op) => {
    //             match op {
    //                 &Op::BlockOpen
    //             }
    //         },
            
    //     }
    // }
}

impl convert::From<Vec<Token>> for Parser {
    fn from(tokens: Vec<Token>) -> Parser {
        Parser {
            token_stream: ParserList(tokens.into_iter().collect()),
            token_index: 0,
            ast: Ast::new()
        }
    }
}