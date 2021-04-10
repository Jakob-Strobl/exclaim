use std::convert;
use std::result;
use std::collections::LinkedList;
use std::ops::{
    Deref, 
    DerefMut
};

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

    /// Returns the token removed from the head of the list 
    /// If you see: let _ = parser.consume(), that means we needed to consume the Token, but the token isnt needed in the AST.
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
                &TokenKind::StringLiteral => Ok(Some(Node::Text(Parser::text(parser)))),
                &TokenKind::Operator(op) => {
                    match op {
                        Op::BlockOpen => Ok(Some(Node::Block(Parser::block(parser)?))),
                        _ => Err(ParserError::from(format!("Unexpected operator found: {:?}", op)))
                    }
                },
                _ => Err(ParserError::from(ErrorKind::Unimplemented))
            }
        } else {
            Ok(None)
        }
    }

    fn text(parser: &mut Parser) -> TextNode {
        let text_node = TextNode::new(parser.consume()); 
        text_node
    }

    /// Parses a Block := {{ BLOCK_STMT }}
    /// parser: the current Parser context
    /// block: the open block (rest of the fields needs to be parsed)
    /// Warning: We should know the next token is Operator(OpenBlock) before calling this function
    fn block(parser: &mut Parser) -> Result<BlockNode> {
        // Parse block stmt field
        fn parse_stmt(parser: &mut Parser) -> Result<StmtNode> {
            Parser::stmt(parser)
        }

        // Parse block close field
        fn parse_close(parser: &mut Parser) -> Result<()> {
            if let Some(token) = parser.peek() {
                match token.kind() {
                    &TokenKind::Operator(op) => {
                        match op {
                            Op::BlockClose => {
                                let _ = parser.consume();
                                Ok(())
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

        let _ = parser.consume(); // Open Block Operator
        let stmt = parse_stmt(parser)?;
        let _ = parse_close(parser)?;

        let block = BlockNode::new(stmt);
        Ok(block)
    }

    fn stmt(parser: &mut Parser) -> Result<StmtNode> {
        fn parse_action(parser: &mut Parser) -> Result<Token> {
            if let Some(token) = parser.peek() {
                match token.kind() {
                    &TokenKind::Action(_) => Ok(parser.consume()),
                    _ => Err(ParserError::from(format!("Unexpected token: {:?}, expected an Action Token", token)))
                }
            } else {
                Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
            }
        }

        fn parse_expr(parser: &mut Parser) -> OptionalResult<Expression> {
            Parser::expr(parser)
        }

        let action = parse_action(parser)?;
        let expr = parse_expr(parser)?;
        let stmt = StmtNode::new(action, expr);
        Ok(stmt)
    }

    fn expr(parser: &mut Parser) -> OptionalResult<Expression> {
        // Lets just parse Literal Expressions for now :D
        if let Some(token) = parser.peek() {
            match token.kind() {
                &TokenKind::NumberLiteral(_) | &TokenKind::StringLiteral => Ok(Some(Expression::Literal(Parser::expr_lit(parser)))),
                &TokenKind::Label => Ok(Some(Expression::Reference(Parser::expr_ref(parser)?))),
                &TokenKind::Operator(op) => {
                    match op {
                        Op::BlockClose => Ok(None),
                        _ => Err(ParserError::from(ErrorKind::Unimplemented)),
                    }
                }
                _ => Err(ParserError::from(format!("Parser<EXPR>: Unexpected token in expression: {:?}, expected a NumberLiteral or StringLiteral.", token)))
            }
        } else {
            Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        }
    }

    fn expr_lit(parser: &mut Parser) -> LiteralExpression {
        let lit_expr = LiteralExpression::new(parser.consume());
        lit_expr
    }

    fn expr_ref(parser: &mut Parser) -> Result<ReferenceExpression> {
        fn parse_reference(parser: &mut Parser) -> Result<Token> {
            if let Some(token) = parser.peek() {
                match token.kind() {
                    &TokenKind::Label => Ok(parser.consume()),
                    _ => Err(ParserError::from("Parser<EXPR_REF>: Unexpected token, expected a reference (label token)."))
                }
            } else {
                Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
            }
        }

        fn parse_child(parser: &mut Parser) -> OptionalResult<Box<ReferenceExpression>> {
            if let Some(token) = parser.peek() {
                match token.kind() {
                    &TokenKind::Operator(op) => {
                        match op {
                            Op::Dot => {
                                let _ = parser.consume(); // Operator(Dot)
                                Ok(Some(Box::new(Parser::expr_ref(parser)?)))
                            },
                            _ => Ok(None),
                        }
                    },
                    _ => Err(ParserError::from(ErrorKind::Unimplemented))
                }
            } else {
                Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
            }
        }
        
        let reference = parse_reference(parser)?;
        let child = parse_child(parser)?;
        let ref_expr = ReferenceExpression::new(reference, child);
        Ok(ref_expr)
    }
}

impl convert::From<Vec<Token>> for Parser {
    fn from(tokens: Vec<Token>) -> Parser {
        Parser {
            token_stream: ParserList(tokens.into_iter().collect()),
        }
    }
}