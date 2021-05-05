use crate::ast::prelude::*;
use crate::tokens::Token;
use crate::data::traits::Renderable;
use crate::data::DataContext;
use crate::data::Data;

mod scope;
use scope::ScopeContext;

mod runtime;
use runtime::RuntimeContext;

pub mod Runtime {
    use crate::data::transforms;

    use super::*;

    pub fn run(mut ast: Ast) -> Result<String, String> {
        let mut runtime = RuntimeContext::new();

        let mut current_block = ast.head();
        while current_block.is_some() {
            current_block = Runtime::run_block(&mut ast, &mut runtime, current_block)?;
        }

        Ok(runtime.output())
    }

    fn run_block(ast: &mut Ast, runtime: &mut RuntimeContext, block_idx: Option<AstIndex>) -> Result<Option<AstIndex>, String> {
        if let Some(block_idx) = block_idx {
            let element_cell = ast.get(block_idx);
            let mut element_ref = element_cell.borrow_mut();

            match &mut *element_ref {
                AstElement::Block(_, block) => {
                    match block {
                        Block::Text(text, next) => {
                            runtime.render(text);
                            Ok(*next)
                        },
                        Block::CodeEnclosed(stmt_idx, next) => {
                            Runtime::run_stmt(ast, runtime, *stmt_idx)?;
                            Ok(*next)
                        },
                        _ => Err("Runtime Error: Block Variant Unimplemented".to_string()),
                    }
                }
                _ => Err("Runtime Error: Expected a block".to_string()),
            }
        } else {
            Err("Runtime Error: AST ended unexpectedly.".to_string())
        }
    }

    fn run_stmt(ast: &mut Ast, runtime: &mut RuntimeContext, stmt_idx: AstIndex) -> Result<(), String> {
        let element_cell = ast.get(stmt_idx);
        let mut element_ref = element_cell.borrow_mut();

        match &mut *element_ref {
            AstElement::Statement(_, stmt) => {
                match stmt {
                    Statement::Write(_action, expr_idx) => {
                        let element_cell = ast.get(*expr_idx);
                        let mut element_ref = element_cell.borrow_mut();

                        match &mut *element_ref {
                            AstElement::Expression(_, expr) => {
                                match expr {
                                    Expression::Literal(token, transforms_idx) => {
                                        let mut literal = Data::from(token.clone());

                                        // Apply transformations
                                        for transform_idx in transforms_idx {
                                            let transform_cell = ast.get(*transform_idx);
                                            let transform_ref = &mut *transform_cell.borrow_mut();

                                            if let AstElement::Transform(_, transform) = transform_ref {
                                                literal = literal.apply_transform(transform);
                                            }
                                        }

                                        runtime.render(&literal);
                                        Ok(())
                                    },
                                    Expression::Reference(reference, transforms_idx) => {
                                        // TODO handle dot operator references 
                                        let variable = reference.get(0).unwrap();
                                        let variable = variable.label().unwrap();

                                        // We clone the data, because all transformation happen out of place
                                        let mut variable = runtime.get(variable).clone();

                                        // Apply transformations
                                        for transform_idx in transforms_idx {
                                            let transform_cell = ast.get(*transform_idx);
                                            let transform_ref = &mut *transform_cell.borrow_mut();

                                            if let AstElement::Transform(_, transform) = transform_ref {
                                                variable = variable.apply_transform(transform);
                                            }
                                        }

                                        runtime.render(&variable);
                                        Ok(())
                                    }
                                }
                            },
                            _ => Err("Runtime Error: Expected an expression".to_string()),
                        }
                    },
                    Statement::Let(_action, pat_idx, expr_idx) => {
                        let pat_cell = ast.get(*pat_idx);
                        let pat_ref = &mut *pat_cell.borrow_mut();

                        let expr_cell = ast.get(*expr_idx);
                        let expr_ref = &mut *expr_cell.borrow_mut();

                        let mut variables: Vec<(String, Data)> = vec![];

                        match pat_ref {
                            AstElement::Pattern(_, pat) => {
                                match pat {
                                    Pattern::Decleration(decls) => {
                                        for decl in decls {
                                            variables.push((decl.label().unwrap().to_string(), Data::Any));
                                        }
                                    }
                                }
                            },
                            _ => return Err("Runtime Error: Let! expected a pattern".to_string()),
                        };

                        // TODO check pattern matches expression
                        if variables.len() != 1 {
                            return Err("Runtime Error: Let! expects patterns of size 1".to_string());
                        }

                        match expr_ref {
                            AstElement::Expression(_, expr) => {
                                match expr {
                                    Expression::Literal(literal, _) => {
                                        variables.get_mut(0).unwrap().1 = match literal {
                                            Token::StringLiteral(str_lit, _) => Data::String(str_lit.to_string()),
                                            Token::NumberLiteral(num, _) => Data::Uint(*num),
                                            _ => return Err("Runtime Error: Let! token variant unimplemented".to_string()),
                                        };
                                    },
                                    _ => return Err("Runtime Error: Let! expr variant unimplemented".to_string()),
                                }
                            }
                            _ => return Err("Runtime Error: Let! expected an expression".to_string()),
                        }

                        // Add variables to runtime context
                        for (key, value) in variables {
                            runtime.insert(key, value);
                        }

                        Ok(())
                    },
                    _ => Err("Runtime Error: Stmt Variant Unimplemented".to_string()),
                }
            }
            _ => Err("Runtime Error: Expected a statement".to_string()),
        }
    }
}