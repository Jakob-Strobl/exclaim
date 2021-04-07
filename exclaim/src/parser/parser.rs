use std::{convert, fmt::Pointer};
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
type OptionalResult<T> = result::Result<Option<Box<T>>, ParserError>;

struct ParserList<T>(LinkedList<T>);
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

        while !parser.end_of_token_stream() {
            match Parser::start(parser) {
                Ok(node) => {
                    if let Some(node) = node {
                        ast.push_block(node);
                    }
                },
                Err(e) => return Err(e),
            }
        }

        Ok(ast)
    }

    fn start(parser: &mut Parser) -> OptionalResult<dyn Node> {
        if let Some(token) = parser.peek() {
            match token.kind() {
                &TokenKind::StringLiteral => {
                    let mut text_node = Box::new(TextNode::new(parser.consume())); 
                    
                    match Parser::start(parser) {
                        Ok(node) => {
                            if let Some(node) = node {
                                text_node.set_next(node);
                            }
                            Ok(Some(text_node))
                        },
                        Err(e) => Err(e)
                    }
                },
                &TokenKind::Operator(op) => {
                    match op {
                        Op::BlockOpen => { 
                            let mut open_block = Box::new(BlockNode::new(parser.consume()));


                            
                            Ok(Some(open_block))
                        }
                        _ => Err(ParserError::from(format!("Unexpected token found: {:?}", op)))
                    }
                },
                _ => Err(ParserError::from(ErrorKind::Unimplemented))
            }
        } else {
            Ok(None)
        }
    }

    fn start_block(parser: Parser) -> Result<BlockNode> {
        if let Some(token) = parser.peek() {
            Err(ParserError::from(ErrorKind::Unimplemented))
        } else {
            Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        }
    }
}

impl convert::From<Vec<Token>> for Parser {
    fn from(tokens: Vec<Token>) -> Parser {
        Parser {
            token_stream: ParserList(tokens.into_iter().collect()),
            token_index: 0,
        }
    }
}