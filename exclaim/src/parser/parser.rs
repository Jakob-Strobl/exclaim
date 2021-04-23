use std::convert;
use std::result;
use std::collections::LinkedList;
use std::ops::{
    Deref, 
    DerefMut
};

use crate::{ast::prelude::*, common::Location};
use crate::tokens::{
    Token,
    TokenKind,
    Op,
    Action,
};

use super::error::{
    ParserError,
    ErrorKind,
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

impl convert::From<Vec<Token>> for Parser {
    fn from(tokens: Vec<Token>) -> Parser {
        Parser {
            token_stream: ParserList(tokens.into_iter().collect()),
        }
    }
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

    pub fn parse(parser: &mut Parser) -> Result<Ast> {
        let mut ast = Ast::new();
        let mut last_idx: Option<AstIndex> = None;

        while !parser.end_of_token_stream() {
            let new_idx = Parser::start(parser, &mut ast)?;
            match last_idx {
                Some(idx) => {
                    // Get last block so we can set next to current new block_idx
                    let last_block = ast.get_mut(idx).unwrap();
                    if let AstElement::Block(_, block) = last_block {
                        block.set_next(new_idx);
                        last_idx = Some(new_idx);
                    } else {
                        return Err(ParserError::from("Parser<parse>: last_idx does not point to a Block element."));
                    }
                }
                None => {
                    // First Block, Set Head! 
                    ast.set_head(new_idx);
                    last_idx = Some(new_idx);
                }
            }
        }

        Ok(ast)
    }

    fn start(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
        if let Some(token) = parser.peek() {
            match token.kind() {
                TokenKind::StringLiteral => {
                    let text_block = Block::Text(parser.consume(), None);
                    let index = ast.push(text_block);
                    Ok(index)
                }
                _ => Parser::parse_block_code(parser, ast)
                
            }
        } else {
            Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        }
    }

    fn parse_block_code(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
        let _block_open = if let Some(token) = parser.peek() {
            match token.kind() {
                TokenKind::Operator(op) => {
                    match op {
                        Op::BlockOpen => parser.consume(),
                        _ => return Err(ParserError::from("Expected Operator(BlockOpen)")),
                    }
                },
                _ => return Err(ParserError::from("Expected Operator(BlockOpen)")),
            }
        } else {
            return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        };

        let stmt_idx = Parser::parse_stmt(parser, ast)?;

        let _block_close = if let Some(token) = parser.peek() {
            match token.kind() {
                TokenKind::Operator(op) => {
                    match op {
                        Op::BlockClose => parser.consume(),
                        _ => return Err(ParserError::from("Expected Operator(BlockClose)")),
                    }
                },
                _ => return Err(ParserError::from("Expected Operator(BlockClose)")),
            }
        } else {
            return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        };

        // Derive the type of block by the statement
        let statement = ast.get(stmt_idx).unwrap(); // This should exist, since it returns index where it pushed the statement
        let block = if let AstElement::Statement(_, statement) = statement {
            match statement {
                Statement::End(_) => Block::CodeClosing(stmt_idx, None),
                Statement::Write(_, _) => Block::CodeEnclosed(stmt_idx, None),
            }
        } else {
            return Err(ParserError::from("Expected to fetch a statement to derive the block type."));
        };

        Ok(ast.push(block))
    }

    fn parse_stmt(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
        if let Some(token) = parser.peek() {
            match token.kind() {
                TokenKind::Action(action) => {
                    match action {
                        Action::End => {
                            let action = parser.consume();
                            let stmt = Statement::End(action);
                            Ok(ast.push(stmt))
                        },
                        Action::Write => {
                            let action = parser.consume();

                            let expr_idx = if let Some(token) = parser.peek() {
                                match token.kind() {
                                    TokenKind::StringLiteral => {
                                        let literal = parser.consume();
                                        let expression = Expression::Literal(literal);
                                        ast.push(expression)
                                    },
                                    TokenKind::NumberLiteral(_) => return Err(ParserError::from(ErrorKind::Unimplemented)),
                                    TokenKind::Label => return Err(ParserError::from(ErrorKind::Unimplemented)),
                                    _ => return Err(ParserError::from("Expected expressions: Reference, StringLiteral, NumberLiteral")),
                                }
                            } else {
                                return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
                            };

                            let stmt = Statement::Write(action, expr_idx);
                            Ok(ast.push(stmt))
                        }
                        _ => return Err(ParserError::from(ErrorKind::Unimplemented))
                    }
                },
                _ => return Err(ParserError::from("Expected Action to start in Block")),
            }
        } else {
            return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        }
    }

    // pub fn parse(parser: &mut Parser) -> result::Result<Ast, ParserError> {
    //     let mut ast = Ast::new();

    //     while !parser.end_of_token_stream() {
    //         match Parser::start(parser) {
    //             Ok(node) => {
    //                 if let Some(node) = node {
    //                     ast.push_block(node);
    //                 }
    //             },
    //             Err(e) => return Err(e),
    //         }
    //     }

    //     Ok(ast)
    // }

    // fn start(parser: &mut Parser) -> OptionalResult<Node> {
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::StringLiteral => Ok(Some(Node::Text(Parser::text(parser)))),
    //             // TODO _ => Parser::block()
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::BlockOpen => Ok(Some(Node::Block(Parser::block(parser)?))),
    //                     _ => Err(ParserError::from(format!("Unexpected operator found: {:?}", op)))
    //                 }
    //             },
    //             _ => Err(ParserError::from(ErrorKind::Unimplemented))
    //         }
    //     } else {
    //         Ok(None)
    //     }
    // }

    // fn text(parser: &mut Parser) -> TextNode {
    //     let text_node = TextNode::new(parser.consume()); 
    //     text_node
    // }

    // /// Parses a Block := {{ BLOCK_STMT }}
    // /// parser: the current Parser context
    // /// block: the open block (rest of the fields needs to be parsed)
    // /// Warning: We should know the next token is Operator(OpenBlock) before calling this function
    // fn block(parser: &mut Parser) -> Result<BlockNode> {
    //     // Parse block close field
    //     fn parse_close(parser: &mut Parser) -> Result<()> {
    //         if let Some(token) = parser.peek() {
    //             match token.kind() {
    //                 &TokenKind::Operator(op) => {
    //                     match op {
    //                         Op::BlockClose => {
    //                             let _ = parser.consume();
    //                             Ok(())
    //                         },
    //                         _ => Err(ParserError::from(format!("Unexpected operator: {:?}, expected BlockClose operator.", op)))
    //                     }
    //                 },
    //                 _ => Err(ParserError::from(format!("Unexpected token: {:?}, expected Operator(BlockClose) token.", token))),
    //             }
    //         } else {
    //             Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //         }
    //     }

    //     let _ = parser.consume(); // Open Block Operator
    //     let stmt = Parser::stmt(parser)?;
    //     let _ = parse_close(parser)?;

    //     let block = BlockNode::new(stmt);
    //     Ok(block)
    // }

    // fn stmt(parser: &mut Parser) -> Result<Statement> {
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Action(action) => {
    //                 match action {
    //                     Action::End => Parser::stmt_end(parser),
    //                     Action::Let => Parser::stmt_let(parser),
    //                     Action::Render => Parser::stmt_render(parser),
    //                     Action::Write => Parser::stmt_write(parser),
    //                 }
    //             },
    //             _ => Err(ParserError::from(format!("Unexpected token: {:?}, expected an Action Token", token)))
    //         }
    //     } else {
    //         Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }
    // }

    // fn stmt_end(parser: &mut Parser) -> Result<Statement> {
    //     let action = parser.consume(); // Action End
    //     let expr = Parser::expr(parser)?;
    //     Ok(Statement::Simple(SimpleStatement::new(action, expr)))
    // }

    // fn stmt_let(parser: &mut Parser) -> Result<Statement> {
    //     let _ = parser.consume(); // Action Let; We don't need the action since it's implicitly defined in the type 
    //     let pattern = Parser::pattern(parser)?.unwrap(); // TODO handle unwrap properly 

    //     // Parse assign operator: =
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::Assign => {
    //                         let _ = parser.consume(); // assign operator
    //                     },
    //                     _ => return Err(ParserError::from("Expected assign operator.")),
    //                 }
    //             },
    //             _ => return Err(ParserError::from("Expected Operator(Assign).")),
    //         }
    //     } else {
    //         return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }

    //     let expr = Parser::expr(parser)?.unwrap(); // There should be an expression
    //     // TODO better error reporting (I have a lot of errors to fix...)

    //     Ok(Statement::Let(LetStatement::new(pattern, expr)))
    // }

    // fn stmt_render(parser: &mut Parser) -> Result<Statement> {
    //     let _ = parser.consume(); // Action render; We don't need the action since it's implicitly defined in the type 
    //     let pattern = Parser::pattern(parser)?;
    //     // Return empty Render Stmt if there is no pattern given 
    //     if pattern.is_none() {
    //         return Ok(Statement::Render(RenderStatement::new(None, None)))
    //     }

    //     // Parse each operator: :
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::Each => {
    //                         let _ = parser.consume(); // each operator
    //                     },
    //                     _ => return Err(ParserError::from("Expected each operator."))
    //                 }
    //             },
    //             _ => return Err(ParserError::from("Expected Operator(Each).")),
    //         }
    //     } else {
    //         return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }

    //     let expr = Parser::expr(parser)?; // There should be an expression
    //     // TODO handle unwrap properly 

    //     Ok(Statement::Render(RenderStatement::new(pattern, expr)))
    // }

    // fn stmt_write(parser: &mut Parser) -> Result<Statement> {
    //     let action = parser.consume(); // Action Write
    //     let expr = Parser::expr(parser)?;
    //     Ok(Statement::Simple(SimpleStatement::new(action, expr)))
    // }

    // fn pattern(parser: &mut Parser) -> OptionalResult<Pattern> {
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             TokenKind::Label => Ok(Some(Parser::pattern_simple(parser)?)),
    //             _ => Parser::pattern_tuple(parser),
    //         }
    //     } else {
    //         Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }
    // }

    // fn pattern_simple(parser: &mut Parser) -> Result<Pattern> {
    //     let decl = parser.consume(); // We know it's a label
    //     Ok(Pattern::Simple(SimplePattern::new(decl)))
    // }

    // fn pattern_tuple(parser: &mut Parser) -> OptionalResult<Pattern> {
    //     // Parse Open Parenthesis 
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::ParenOpen => { 
    //                         let _ = parser.consume(); // ParenOpen: (
    //                     },
    //                     _ => return Ok(None)
    //                 }
    //             },
    //             _ => return Ok(None)
    //         }
    //     } else {
    //         return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }

    //     // Parse Declerations in Tuple Pattern
    //     let mut decls: Vec<Token> = Vec::new();

    //     loop {
    //         let decl = Parser::parse_label(parser)?;
    //         decls.push(decl);

    //         // If there isnt a following comma, break
    //         if Parser::parse_comma(parser)?.is_none() {
    //             break;
    //         }
    //     }

    //     // Parse Close Parenthesis 
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::ParenClose => { 
    //                         let _ = parser.consume(); // ParenClose: )
    //                     },
    //                     _ => return Err(ParserError::from(format!("Expected CloseParen operator, found {:?}", token)))
    //                 }
    //             },
    //             _ => return Err(ParserError::from("Expected Operator(CloseParen)"))
    //         }
    //     } else {
    //         return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }

    //     Ok(Some(Pattern::Tuple(TuplePattern::new(decls))))
    // }
    
    // fn expr(parser: &mut Parser) -> OptionalResult<Expression> {
    //     // Lets just parse Literal Expressions for now :D
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::NumberLiteral(_) | &TokenKind::StringLiteral => Ok(Some(Expression::Literal(Parser::expr_lit(parser)?))),
    //             &TokenKind::Label => Ok(Some(Expression::Reference(Parser::expr_ref(parser)?))),
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::BlockClose => Ok(None),
    //                     _ => Err(ParserError::from(ErrorKind::Unimplemented)),
    //                 }
    //             }
    //             _ => Err(ParserError::from(format!("Parser<EXPR>: Unexpected token in expression: {:?}, expected a NumberLiteral or StringLiteral.", token)))
    //         }
    //     } else {
    //         Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }
    // }

    // fn expr_lit(parser: &mut Parser) -> Result<LiteralExpression> {
    //     let literal = parser.consume();
    //     let pipe = Parser::expr_pipe(parser)?;
    //     let lit_expr = LiteralExpression::new(literal, pipe);
    //     Ok(lit_expr)
    // }

    // fn expr_ref(parser: &mut Parser) -> Result<ReferenceExpression> {
    //     // Parses REF
    //     fn parse_reference(parser: &mut Parser) -> Result<Token> {
    //         if let Some(token) = parser.peek() {
    //             match token.kind() {
    //                 &TokenKind::Label => Ok(parser.consume()),
    //                 _ => Err(ParserError::from("Parser<EXPR_REF>: Unexpected token, expected a reference (label token)."))
    //             }
    //         } else {
    //             Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //         }
    //     }

    //     // Parses REF_PRIME
    //     fn parse_child(parser: &mut Parser) -> OptionalResult<Box<ReferenceExpression>> {
    //         if let Some(token) = parser.peek() {
    //             match token.kind() {
    //                 &TokenKind::Operator(op) => {
    //                     match op {
    //                         Op::Dot => {
    //                             let _ = parser.consume(); // Operator(Dot)
    //                             Ok(Some(Box::new(Parser::expr_ref(parser)?)))
    //                         },
    //                         _ => Ok(None),
    //                     }
    //                 },
    //                 _ => Err(ParserError::from(ErrorKind::Unimplemented))
    //             }
    //         } else {
    //             Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //         }
    //     }
        
    //     let reference = parse_reference(parser)?;
    //     let child = parse_child(parser)?;
    //     let pipe = Parser::expr_pipe(parser)?;
    //     Ok(ReferenceExpression::new(reference, child, pipe))
    // }

    // fn expr_pipe(parser: &mut Parser) -> OptionalResult<Pipe> {
    //     fn parse_pipe(parser: &mut Parser) -> Result<Pipe> {
    //         let _ = parser.consume(); // Consume Pipe operator |
    //         let call = Parser::call(parser)?;
    //         let next = match Parser::expr_pipe(parser)? {
    //             Some(pipe) => Some(Box::new(pipe)),
    //             None => None,
    //         };

    //         Ok(Pipe::new(call, next))
    //     }

    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::Pipe => Ok(Some(parse_pipe(parser)?)),
    //                     _ => Ok(None)
    //                 }
    //             }
    //             _ => Ok(None)
    //         }
    //     } else {
    //         Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }
    // }

    // fn call(parser: &mut Parser) -> Result<Call> {
    //     let function = if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Label => Ok(parser.consume()),
    //             _ => Err(ParserError::from("Expected a function to be named here! Expected Token Label."))
    //         }
    //     } else {
    //         Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }?;

    //     let arguments = Parser::arguments(parser)?;

    //     Ok(Call::new(function, arguments))
    // }

    // fn arguments(parser: &mut Parser) -> OptionalResult<Arguments> {
    //     fn parse_arg(parser: &mut Parser) -> Result<Expression> {
    //         match Parser::expr(parser)? {
    //             Some(expr) => Ok(expr),
    //             None => Err(ParserError::from("Expected an argument"))
    //         }
    //     }

    //     // Parse Open Parenthesis
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::ParenOpen => { 
    //                         let _ = parser.consume(); // ParenOpen (
    //                     },
    //                     _ => return Ok(None)
    //                 }
    //             },
    //             _ => return Ok(None)
    //         }
    //     } else {
    //         return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }

    //     let mut args: Vec<Expression> = Vec::new();
        
    //     // Parse arguments
    //     loop {
    //         let arg = parse_arg(parser)?;
    //         args.push(arg);

    //         // If there isnt a following comma, break
    //         if Parser::parse_comma(parser)?.is_none() {
    //             break;
    //         }
    //     }
        
    //     // Parse Close Parentheses
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::ParenClose => { 
    //                         let _ = parser.consume(); // ParenClose )
    //                     },
    //                     _ => return Err(ParserError::from("Expected ParenClose operator."))
    //                 }
    //             },
    //             _ => return Err(ParserError::from("Expected Operator(ParenClose)"))
    //         }
    //     };

    //     Ok(Some(Arguments::new(args)))
    // }


    // // Unit Parsing functions
    // fn parse_label(parser: &mut Parser) -> Result<Token> {
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             &TokenKind::Label => {
    //                 Ok(parser.consume())
    //             }
    //             _ => return Err(ParserError::from("Expected a Label Token")),
    //         }
    //     } else {
    //         return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }
    // }

    // fn parse_comma(parser: &mut Parser) -> OptionalResult<Token> {
    //     if let Some(token) = parser.peek() {
    //         match token.kind() {
    //             TokenKind::Operator(op) => {
    //                 match op {
    //                     Op::Comma => {
    //                         Ok(Some(parser.consume()))
    //                     },
    //                     _ => Ok(None),
    //                 }
    //             },
    //             _ => Ok(None),
    //         }
    //     } else {
    //         return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
    //     }
    // }
}