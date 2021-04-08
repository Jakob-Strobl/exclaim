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
    Op,
    Action
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
                    Ok(Some(Node::Text(text_node)))
                },
                &TokenKind::Operator(op) => {
                    match op {
                        Op::BlockOpen => { 
                            let block = BlockNode::new(parser.consume());
                            match Parser::block(parser, block) {
                                Ok(block) => Ok(Some(Node::Block(block))),
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

    /// Parses a Block := {{ BLOCK_STMT }}
    /// parser: the current Parser context
    /// block: the open block (rest of the fields needs to be parsed)
    fn block(parser: &mut Parser, block: BlockNode) -> Result<BlockNode> {
        // Parse block stmt field
        fn parse_stmt(parser: &mut Parser, mut block: BlockNode) -> Result<BlockNode> {
            let stmt = Parser::stmt(parser)?;
            block.set_stmt(stmt);
            Ok(block)
        }

        // Parse block close field
        fn parse_close(parser: &mut Parser, mut block: BlockNode) -> Result<BlockNode> {
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

        let block = parse_stmt(parser, block)?;
        let block = parse_close(parser, block);
        block
    }

    fn stmt(parser: &mut Parser) -> Result<StmtNode> {
        fn parse_action(parser: &mut Parser) -> Result<StmtNode> {
            if let Some(token) = parser.peek() {
                match token.kind() {
                    &TokenKind::Action(_) => {
                        let stmt = StmtNode::new(parser.consume());
                        Ok(stmt)
                    },
                    _ => Err(ParserError::from(format!("Unexpected token: {:?}, expected an Action Token", token)))
                }
            } else {
                Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
            }
        }

        fn parse_expr(parser: &mut Parser, mut stmt: StmtNode) -> Result<StmtNode> {
            // If the action is not Action::End {{!}}, parse the following expression
            if let TokenKind::Action(action) = stmt.action().kind() {
                if *action != Action::End {
                    let expr = Parser::expr(parser)?;
                    stmt.set_expr(expr);
                }
            } else {
                return Err(ParserError::from("Parser<STMT> somehow we are parsing an expression of an action-less statement?"));
            }
            Ok(stmt)
        }

        let stmt = parse_action(parser)?;
        let stmt = parse_expr(parser, stmt);
        stmt
    }

    fn expr(parser: &mut Parser) -> Result<Expression> {
        // Lets just parse Literal Expressions for now :D
        if let Some(token) = parser.peek() {
            match token.kind() {
                &TokenKind::NumberLiteral(_) | &TokenKind::StringLiteral => {
                    let lit_expr = LiteralExpression::new(parser.consume());
                    Ok(Expression::Literal(lit_expr))
                },
                _ => Err(ParserError::from(format!("Parser<EXPR>: Unexpected token in expression: {:?}, expected a NumberLiteral or StringLiteral.", token)))
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
        }
    }
}