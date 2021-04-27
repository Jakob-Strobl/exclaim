use std::convert;
use std::result;
use std::collections::LinkedList;
use std::ops::{
    Deref, 
    DerefMut
};

use crate::ast::prelude::*;
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

// Keeping this here for future reference
// I didnt realize that blocks { ... } are also resolved as expressions until I read this answer: https://stackoverflow.com/questions/27329653/writing-a-macro-that-contains-a-match-body
// macro_rules! match_token {
//     // Pass the token you want to match, then follow with arms for a match on token.kind()
//     {($token:expr) { $($pattern:pat => $expression:expr),* }} => {
//         if let Some(token) = $token {
//             match token.kind() {
//                 $(
//                     $pattern => $expression
//                 ),*
//             }
//         } else {
//             return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
//         }
//     };
// }

macro_rules! unwrap_token {
    ($token:expr) => {
        if let Some(token) = $token {
            token
        } else {
            return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        }
    };
}


// Parsing functions
impl Parser {

    pub fn parse(parser: &mut Parser) -> Result<Ast> {
        let mut ast = Ast::new();
        let mut last_idx: Option<AstIndex> = None;

        while !parser.end_of_token_stream() {
            let new_idx = Parser::parse_block(parser, &mut ast)?;
            match last_idx {
                Some(idx) => {
                    // Get last block so we can set next to current new block_idx
                    let last_block = ast.get(idx);
                    if let AstElement::Block(_, block) = &mut *last_block.borrow_mut() {
                        block.set_next(new_idx);
                        last_idx = Some(new_idx);
                    } else {
                        return Err(ParserError::from("Parser<parse>: last_idx does not point to a Block element."));
                    }
                }
                None => {
                    // First Block -> Set Head! 
                    ast.set_head(new_idx);
                    last_idx = Some(new_idx);
                }
            }
        }

        Ok(ast)
    }

    fn parse_block(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
        let token = unwrap_token!(parser.peek());
        match token.kind() {
            TokenKind::StringLiteral => {
                let text_block = Block::Text(parser.consume(), None);
                let index = ast.push(text_block);
                Ok(index)
            },
            _ => Parser::parse_block_code(parser, ast)
        }
    }

    fn parse_block_code(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
        let token = unwrap_token!(parser.peek());
        let _block_open = match token.kind() {
            TokenKind::Operator(op) => {
                match op {
                    Op::BlockOpen => parser.consume(),
                    _ => return Err(ParserError::from("Expected Operator(BlockOpen)")),
                }
            },
            _ => return Err(ParserError::from("Expected Operator(BlockOpen)")),
        };

        let stmt_idx = Parser::parse_stmt(parser, ast)?;

        let token = unwrap_token!(parser.peek());
        let _block_close = match token.kind() {
            TokenKind::Operator(op) => {
                match op {
                    Op::BlockClose => parser.consume(),
                    _ => return Err(ParserError::from("Expected Operator(BlockClose)")),
                }
            },
            _ => return Err(ParserError::from("Expected Operator(BlockClose)")),
        };

        // Derive the type of block by the statement
        let statement = ast.get(stmt_idx);
        let block = if let AstElement::Statement(_, statement) = &*statement.borrow_mut() {
            match statement {
                Statement::End(_) => Block::CodeClosing(stmt_idx, None),
                Statement::Let(_, _, _) => Block::CodeEnclosed(stmt_idx, None),
                Statement::Render(_, _, _) => Block::CodeUnclosed(stmt_idx, vec![], None), // Scope is filled in during semantic analysis
                Statement::Write(_, _) => Block::CodeEnclosed(stmt_idx, None)
            }
        } else {
            return Err(ParserError::from("Expected to fetch a statement to derive the block type."));
        };

        Ok(ast.push(block))
    }

    fn parse_stmt(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
        let token = unwrap_token!(parser.peek());
        match token.kind() {
            TokenKind::Action(action) => {
                match action {
                    Action::End => {
                        let action = parser.consume();
                        let stmt = Statement::End(action);
                        Ok(ast.push(stmt))
                    },
                    Action::Let => {
                        let action = parser.consume();
                        let pattern = Parser::parse_pattern_decleration(parser, ast)?;

                        // Parse Operator(assign)
                        let token = unwrap_token!(parser.peek());
                        let _assign = match token.kind() {
                            TokenKind::Operator(op) => {
                                match op {
                                    Op::Assign => parser.consume(),
                                    _ => return Err(ParserError::from("Expected assign operator.")),
                                }
                            },
                            _ => return Err(ParserError::from("Expected Operator(Assign).")),
                        };

                        let expr = Parser::parse_expr(parser, ast)?;
                        let stmt = Statement::Let(action, pattern, expr);
                        Ok(ast.push(stmt))
                    },
                    Action::Render => {
                        let action = parser.consume();
                        let pattern = Parser::parse_pattern_decleration(parser, ast)?;
                    
                        // Parse Operator(each)
                        let token = unwrap_token!(parser.peek());
                        let _each = match token.kind() {
                            TokenKind::Operator(op) => {
                                match op {
                                    Op::Each => parser.consume(),
                                    _ => return Err(ParserError::from("Expected each operator.")),
                                }
                            },
                            _ => return Err(ParserError::from("Expected Operator(Each)"))
                        };

                        let expr = Parser::parse_expr(parser, ast)?;
                        let stmt = Statement::Render(action, pattern, expr);
                        Ok(ast.push(stmt))
                    },
                    Action::Write => {
                        let action = parser.consume();
                        let expr = Parser::parse_expr(parser, ast)?;
                        let stmt = Statement::Write(action, expr);
                        Ok(ast.push(stmt))
                    }
                    _ => return Err(ParserError::from(ErrorKind::Unimplemented))
                }
            },
            _ => return Err(ParserError::from("Expected Action to start in Block")),
        }
    }

    fn parse_expr(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
        let token = unwrap_token!(parser.peek());
        match token.kind() {
            TokenKind::StringLiteral => {
                let literal = parser.consume();
                let transforms = Parser::parse_tranforms(parser, ast)?;
                let expression = Expression::Literal(literal, transforms);
                Ok(ast.push(expression))
            },
            TokenKind::NumberLiteral(_) => {
                let literal = parser.consume();
                let transforms = Parser::parse_tranforms(parser, ast)?;
                let expression = Expression::Literal(literal, transforms);
                Ok(ast.push(expression))
            },
            TokenKind::Label => {
                let mut ref_list = vec![parser.consume()];
                
                // Collect dot operated references
                loop {
                    // Check for dot operator
                    let token = unwrap_token!(parser.peek());
                    match token.kind() {
                        TokenKind::Operator(op) => {
                            match op {
                                Op::Dot => {
                                    // Expect a label token 
                                    let _dot = parser.consume();

                                    let token = unwrap_token!(parser.peek());
                                    let label = match token.kind() {
                                        TokenKind::Label => parser.consume(),
                                        _ => return Err(ParserError::from("Expected a label after dot operator"))
                                    };

                                    ref_list.push(label);
                                },
                                _ => break,
                            }
                        }
                        _ => break,
                    }
                }

                let transforms = Parser::parse_tranforms(parser, ast)?;
                let expression = Expression::Reference(ref_list, transforms);
                Ok(ast.push(expression))
            },
            _ => return Err(ParserError::from("Expected expressions: Reference, StringLiteral, NumberLiteral")),
        }
    }

    fn parse_tranforms(parser: &mut Parser, ast: &mut Ast) -> Result<Vec<AstIndex>> {
        let mut transforms: Vec<AstIndex> = vec![];

        // collect as many transforms as possible 
        loop {
            let token = unwrap_token!(parser.peek());
            let _pipe = match token.kind() {
                TokenKind::Operator(op) => {
                    match op {
                        Op::Pipe => parser.consume(), // Pipe operator |
                        _ => break,
                    }
                }
                _ => break,
            };
            
            let token = unwrap_token!(parser.peek());
            let label = match token.kind() {
                TokenKind::Label => parser.consume(), // Label
                _ => return Err(ParserError::from("Expected transform label after Pipe Operator.")),
            };

            // Parse arguments
            let mut arguments: Vec<AstIndex> = vec![];
            let token = unwrap_token!(parser.peek());
            match token.kind() {
                TokenKind::Operator(op) => {
                    match op {
                        Op::ParenOpen => {
                            let _paren_open = parser.consume(); // Paren open (

                            loop {
                                let argument = Parser::parse_expr(parser, ast)?;
                                arguments.push(argument);

                                // Determine if a comma or an close parenthesis
                                let token = unwrap_token!(parser.peek());
                                match token.kind() {
                                    TokenKind::Operator(op) => {
                                        match op {
                                            Op::Comma => {
                                                let _comma = parser.consume();
                                                continue; // More arguments to parse!
                                            },
                                            Op::ParenClose => {
                                                let _close_paren = parser.consume();
                                                break; // End of argument list 
                                            },
                                            _ => return Err(ParserError::from("Expected a comma or close parenthesis to complete an argument list."))
                                        }
                                    },
                                    _ => return Err(ParserError::from("Expected a comma or close parenthesis to complete an argument list."))
                                }
                            }
                        },
                        _ => (), // do nothing
                    }
                },
                _ => (), // do nothing
            }
            
            
            // Create transform and add to list of transforms 
            let transform = Transform::new(label, arguments);
            let index = ast.push(transform);
            transforms.push(index);
        }

        Ok(transforms)
    }

    fn parse_pattern_decleration(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
        // Parse Pattern 
        let token = unwrap_token!(parser.peek());
        let decls = match token.kind() {
            TokenKind::Label => vec![parser.consume()],
            TokenKind::Operator(op) => {
                match op {
                    Op::ParenOpen => {
                        let _open_paren = parser.consume();

                        // Parse declerations 
                        let mut decls: Vec<Token> = vec![];
                        loop {
                            let token = unwrap_token!(parser.peek());
                            let decl = match token.kind() {
                                TokenKind::Label => parser.consume(),
                                _ => return Err(ParserError::from("Expected label for decleration in Pattern"))
                            };

                            decls.push(decl);

                            // Determine if end of pattern or more declerations to parse
                            let token = unwrap_token!(parser.peek());
                            match token.kind() {
                                TokenKind::Operator(op) => {
                                    match op {
                                        Op::Comma => {
                                            let _comma = parser.consume();
                                            continue; // More declerations!
                                        },
                                        Op::ParenClose => {
                                            let _close_paren = parser.consume();
                                            break; // End of pattern
                                        }
                                        _ => return Err(ParserError::from("Expected either a Close Parenthesis to end the Pattern, or a comma to continue the pattern."))
                                    }
                                },
                                _ => return Err(ParserError::from("Expected either a Close Parenthesis to end the Pattern, or a comma to continue the pattern."))
                            }
                        }

                        decls
                    },
                    _ => return Err(ParserError::from("Expected either a Close Parenthesis to end the Pattern, or a comma to continue the pattern."))
                }
            }
            _ => return Err(ParserError::from("Expected a Decleration Pattern to assign an expression to in a let! statement."))
        };
        let pattern = Pattern::Decleration(decls);
        Ok(ast.push(pattern))
    }
}