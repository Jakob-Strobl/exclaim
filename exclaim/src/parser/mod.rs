use std::result;
use std::collections::LinkedList;

use crate::ast::prelude::*;
use crate::tokens::*;

pub mod error;
use error::*;

type Result<T> = result::Result<T, ParserError>;

pub struct Parser(LinkedList<Token>);

// Methods
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser(tokens.into_iter().collect())
    }

    fn peek(&self) -> Option<&Token> {
        self.0.front()
    }

    /// Returns the token removed from the head of the list 
    /// If you see: let _ = parser.consume(), that means we needed to consume the Token, but the token isnt needed in the AST.
    fn consume(&mut self) -> Token {
        self.0.pop_front().unwrap()
    }

    fn end_of_token_stream(&self) -> bool {
        self.0.is_empty()
    }
}

macro_rules! unwrap_token {
    ($token:expr) => {
        if let Some(token) = $token {
            token
        } else {
            return Err(ParserError::from(ErrorKind::UnexpectedEndOfTokenStream))
        }
    };
}

pub fn run(tokens: Vec<Token>) -> Result<Ast> {
    let mut parser = Parser::new(tokens);
    parse(&mut parser)
}

fn parse(parser: &mut Parser) -> Result<Ast> {
    let mut ast = Ast::new();
    let mut last_idx: Option<AstIndex> = None;

    while !parser.end_of_token_stream() {
        let new_idx = parse_block(parser, &mut ast)?;
        match last_idx {
            Some(idx) => {
                // Get last block so we can set next to current new block_idx
                let last_block = ast.get(idx);
                if let AstElement::Block(_, block) = &mut *last_block.borrow_mut() {
                    block.set_next(new_idx);
                    last_idx = Some(new_idx);
                } else {
                    return Err(ParserError::from("Parser<parse>: last_idx does not point to a Block element."));
                };
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
    match token {
        Token::StringLiteral(_, _) => {
            let text_block = Block::Text(parser.consume(), None);
            let index = ast.push(text_block);
            Ok(index)
        },
        _ => parse_block_code(parser, ast)
    }
}

fn parse_block_code(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
    let token = unwrap_token!(parser.peek());
    let _block_open = match token {
        Token::Operator(op, _) => {
            match op {
                Op::BlockOpen => parser.consume(),
                _ => return Err(ParserError::from("Expected Operator(BlockOpen)")),
            }
        },
        _ => return Err(ParserError::from("Expected Operator(BlockOpen)")),
    };

    let statement_idx = parse_statement(parser, ast)?;

    let token = unwrap_token!(parser.peek());
    let _block_close = match token {
        Token::Operator(op, _) => {
            match op {
                Op::BlockClose => parser.consume(),
                _ => return Err(ParserError::from("Expected Operator(BlockClose)")),
            }
        },
        _ => return Err(ParserError::from("Expected Operator(BlockClose)")),
    };

    // Derive the type of block by the statement
    // TODO-ALPHA this code displays a design flaw in the data structures; highlights the unclear relationship between blocks and statements.
    let statement_cell = ast.get(statement_idx);
    let block = if let AstElement::Statement(_, statement) = &*statement_cell.borrow_mut() {
        match statement {
            Statement::End(_) => Block::CodeClosing(statement_idx, None),
            Statement::Let(_, _, _) => Block::CodeEnclosed(statement_idx, None),
            Statement::Render(_, _, _) => Block::CodeUnclosed(statement_idx, vec![], None), // Scope is filled in during semantic analysis
            Statement::Write(_, _) => Block::CodeEnclosed(statement_idx, None)
        }
    } else {
        return Err(ParserError::from("Expected to fetch a statement to derive the block type."));
    };

    Ok(ast.push(block))
}

fn parse_statement(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
    let token = unwrap_token!(parser.peek());
    match token {
        Token::Action(action, _) => {
            match action {
                Action::End => {
                    let action = parser.consume();
                    let statement = Statement::End(action);
                    Ok(ast.push(statement))
                },
                Action::Let => {
                    let action = parser.consume();
                    let pattern = parse_pattern_decleration(parser, ast)?;

                    // Parse Operator(assign)
                    let token = unwrap_token!(parser.peek());
                    let _assign = match token {
                        Token::Operator(op, _) => {
                            match op {
                                Op::Assign => parser.consume(),
                                _ => return Err(ParserError::from("Expected assign operator.")),
                            }
                        },
                        _ => return Err(ParserError::from("Expected Operator(Assign).")),
                    };

                    let expression = parse_expression(parser, ast)?;
                    let statement = Statement::Let(action, pattern, expression);
                    Ok(ast.push(statement))
                },
                Action::Render => {
                    let action = parser.consume();
                    let pattern = parse_pattern_decleration(parser, ast)?;
                
                    // Parse Operator(each)
                    let token = unwrap_token!(parser.peek());
                    let _each = match token {
                        Token::Operator(op, _) => {
                            match op {
                                Op::Each => parser.consume(),
                                _ => return Err(ParserError::from("Expected each operator.")),
                            }
                        },
                        _ => return Err(ParserError::from("Expected Operator(Each)"))
                    };

                    let expression = parse_expression(parser, ast)?;
                    let statement = Statement::Render(action, pattern, expression);
                    Ok(ast.push(statement))
                },
                Action::Write => {
                    let action = parser.consume();
                    let expression = parse_expression(parser, ast)?;
                    let statement = Statement::Write(action, expression);
                    Ok(ast.push(statement))
                }
            }
        },
        _ => return Err(ParserError::from("Expected Action to start in Block")),
    }
}

fn parse_expression(parser: &mut Parser, ast: &mut Ast) -> Result<AstIndex> {
    let token = unwrap_token!(parser.peek());
    match token {
        Token::StringLiteral(_, _) => {
            let literal = parser.consume();
            let transforms = parse_tranforms(parser, ast)?;
            let expression = Expression::Literal(literal, transforms);
            Ok(ast.push(expression))
        },
        Token::NumberLiteral(_, _) => {
            let literal = parser.consume();
            let transforms = parse_tranforms(parser, ast)?;
            let expression = Expression::Literal(literal, transforms);
            Ok(ast.push(expression))
        },
        Token::Label(_, _) => {
            let mut ref_list = vec![parser.consume()];
            
            // Collect dot operated references
            loop {
                // Check for dot operator
                let token = unwrap_token!(parser.peek());
                match token {
                    Token::Operator(op, _) => {
                        match op {
                            Op::Dot => {
                                // Expect a label token 
                                let _dot = parser.consume();

                                let token = unwrap_token!(parser.peek());
                                let label = match token {
                                    Token::Label(_, _) => parser.consume(),
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

            let transforms = parse_tranforms(parser, ast)?;
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
        let _pipe = match token {
            Token::Operator(op, _) => {
                match op {
                    Op::Pipe => parser.consume(), // Pipe operator |
                    _ => break,
                }
            }
            _ => break,
        };
        
        let token = unwrap_token!(parser.peek());
        let label = match token {
            Token::Label(_, _) => parser.consume(), // Label
            _ => return Err(ParserError::from("Expected transform label after Pipe Operator.")),
        };

        // Parse arguments
        let mut arguments: Vec<AstIndex> = vec![];
        let token = unwrap_token!(parser.peek());
        match token {
            Token::Operator(op, _) => {
                match op {
                    Op::ParenOpen => {
                        let _paren_open = parser.consume(); // Paren open (
                        
                        // Parse argument list
                        loop {
                            let argument = parse_expression(parser, ast)?;
                            arguments.push(argument);

                            // Check if next token is a comma or an close parenthesis
                            let token = unwrap_token!(parser.peek());
                            match token {
                                Token::Operator(op, _) => {
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
    let decls = match token {
        Token::Label(_, _) => vec![parser.consume()],
        Token::Operator(op, _) => {
            match op {
                Op::ParenOpen => {
                    let _open_paren = parser.consume();

                    // Parse declerations 
                    let mut decls: Vec<Token> = vec![];
                    loop {
                        let token = unwrap_token!(parser.peek());
                        let decl = match token {
                            Token::Label(_, _) => parser.consume(),
                            _ => return Err(ParserError::from("Expected label for decleration in Pattern"))
                        };

                        decls.push(decl);

                        // Determine if end of pattern or more declerations to parse
                        let token = unwrap_token!(parser.peek());
                        match token {
                            Token::Operator(op, _) => {
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