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

type Result<T> = result::Result<T, ParserError>;
type OptionalResult<T> = result::Result<Option<T>, ParserError>;

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

// Parsing functions
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

    fn start(parser: &mut Parser) -> OptionalResult<Node> {
        if let Some(token) = parser.peek() {
            match token.kind() {
                &TokenKind::StringLiteral => {
                    let text_node = TextNode::new(parser.consume()); 
                    Ok(Some(Node::TextNode(text_node)))
                },
                &TokenKind::Operator(op) => {
                    match op {
                        Op::BlockOpen => { 
                            let block = BlockNode::new(parser.consume());
                            match Parser::block(parser, block) {
                                Ok(block) => Ok(Some(Node::BlockNode(block))),
                                Err(e) => Err(e)
                            }
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

    fn block(parser: &mut Parser, block: BlockNode) -> Result<BlockNode> {
        // Parse block stmt field
        fn block_stmt(parser: &mut Parser, mut block: BlockNode) -> Result<BlockNode> {
            match Parser::block_stmt(parser) {
                Ok(node) => {
                    match node {
                        Node::TextNode(text) => {
                            block.set_text(text);
                            Ok(block)
                        },
                        _ => return Err(ParserError::from(format!("Parser::start_block returned unexpected node: {:?}", node)))
                    }
                },
                Err(e) => return Err(e)
            }
        }

        // Parse block close
        fn block_close(parser: &mut Parser, mut block: BlockNode) -> Result<BlockNode> {
            if let Some(token) = parser.peek() {
                match token.kind() {
                    &TokenKind::Operator(op) => {
                        match op {
                            Op::BlockClose => {
                                block.set_close(parser.consume());
                                Ok(block)
                            },
                            _ => Err(ParserError::from(format!("Unexpected operator: {:?}", op)))
                        }
                    },
                    _ => Err(ParserError::from(format!("Unexpected token: {:?}", token))),
                }
            } else {
                Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
            }
        }

        let block = block_stmt(parser, block)?;
        block_close(parser, block)
    }

    fn block_stmt(parser: &mut Parser) -> Result<Node> {
        if let Some(token) = parser.peek() {
            match token.kind() {
                &TokenKind::StringLiteral => {
                    let string_node = TextNode::new(parser.consume());

                    Ok(Node::TextNode(string_node))
                },
                _ => Err(ParserError::from(ErrorKind::Unimplemented))
            }
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