use crate::ast::prelude::*;
use crate::data::traits::Renderable;
use crate::data::DataContext;
use crate::data::Data;

mod scope;
use scope::ScopeContext;

mod runtime;
use runtime::RuntimeContext;


pub fn run(mut ast: Ast) -> Result<String, String> {
    let mut runtime = RuntimeContext::new();

    let mut current_block = ast.head();
    while current_block.is_some() {
        current_block = run_block(&mut ast, &mut runtime, current_block)?;
    }

    Ok(runtime.output())
}

fn run_block(ast: &mut Ast, runtime: &mut RuntimeContext, block: Option<AstIndex>) -> Result<Option<AstIndex>, String> {
    if let Some(block) = block {
        let element_cell = ast.get(block);
        let mut element_ref = element_cell.borrow_mut();

        match &mut *element_ref {
            AstElement::Block(_, block) => {
                match block {
                    Block::Text(text, next) => {
                        runtime.render(text);
                        Ok(*next)
                    },
                    Block::CodeEnclosed(stmt, next) => {
                        run_stmt(ast, runtime, *stmt)?;
                        Ok(*next)
                    },
                    Block::CodeUnclosed(stmt, scope, next) => {
                        let stmt_cell = ast.get(*stmt);
                        let mut stmt_ref = stmt_cell.borrow_mut();

                        match &mut *stmt_ref {
                            AstElement::Statement(_, stmt) => {
                                match stmt {
                                    Statement::Render(_action, pattern, expression) => {
                                        // Left hand side of assignment - build declerations
                                        let pattern = run_pattern(ast, runtime, *pattern)?;
                                        
                                        // Right hand side of assignment - compute expressions and get values
                                        let values = run_expression(ast, runtime, *expression)?;

                                        // Open new scope
                                        runtime.open_scope();

                                        // Get iterator from Data variant 
                                        for value in values.into_iter() {
                                            // Insert current value for the iteration
                                            for (decl, data) in pattern.iter().zip(value.clone().into_iter()) {
                                                runtime.insert(decl.to_string(), data);
                                            }

                                            // Run iteration
                                            for nested_block in scope.iter() {
                                                run_block(ast, runtime, Some(*nested_block))?;
                                            }
                                        }

                                        // Close Scope
                                        runtime.close_scope();
                                    },
                                    _ => return Err("Runtime Error: Expected a Render Statement.".to_string()),
                                }
                            },
                            _ => return Err("Runtime Error: Expected a statement.".to_string()),
                        }
                        
                        println!("next is : {:?}", next);
                        Ok(*next)
                    }
                    Block::CodeClosing(_stmt, next) => {
                        Ok(*next)
                    },
                }
            }
            _ => Err("Runtime Error: Expected a block".to_string()),
        }
    } else {
        Err("Runtime Error: AST ended unexpectedly.".to_string())
    }
}

fn run_stmt(ast: &mut Ast, runtime: &mut RuntimeContext, stmt: AstIndex) -> Result<(), String> {
    let element_cell = ast.get(stmt);
    let mut element_ref = element_cell.borrow_mut();

    match &mut *element_ref {
        AstElement::Statement(_, stmt) => {
            match stmt {
                Statement::Write(_action, expression) => {
                    let data = run_expression(ast, runtime, *expression)?;
                    runtime.render(&data);
                    Ok(())
                },
                Statement::Let(_action, pattern, expression) => {
                    // Left hand side of assignment
                    let mut pattern = run_pattern(ast, runtime, *pattern)?;

                    // Right hand side of assignment - compute expressions and get values
                    let value = run_expression(ast, runtime, *expression)?;

                    // Add variables to runtime context
                    if pattern.len() == value.len() || pattern.len() != 1 {
                        for (key, value) in pattern.into_iter().zip(value) {
                            runtime.insert(key, value);
                        }
                    } else if pattern.len() == 1 {
                        runtime.insert(pattern.pop().unwrap(), value)
                    } else {
                        return Err("Runtime Error: Let! expects pattern does not match expression.".to_string());
                    }

                    Ok(())
                },
                _ => Err("Runtime Error: Stmt Variant Unimplemented".to_string()),
            }
        }
        _ => Err("Runtime Error: Expected a statement".to_string()),
    }
}

fn run_expression(ast: &mut Ast, runtime: &mut RuntimeContext, expression: AstIndex) -> Result<Data, String> {
    let expression_cell = ast.get(expression);
    let expression_ref = &*expression_cell.borrow_mut();

    if let AstElement::Expression(_, expression) = expression_ref {
        match expression {
            Expression::Literal(literal, transforms) => {
                let literal = Data::from(literal.clone());
                let literal = run_transformations(ast, runtime, literal, transforms)?;
                Ok(literal)
            }
            Expression::Reference(reference, transforms) => {
                // TODO handle dot operator references 
                let token = reference.get(0).unwrap();
                let reference_key = token.label().unwrap();

                // We clone the data, because all transformation happen out of place
                let reference = runtime.get(reference_key).clone();

                // Apply transformations
                let reference = run_transformations(ast, runtime, reference, transforms)?;

                Ok(reference)
            }
        }
    } else {
        Err("Runtime: Expected an expression".to_string())
    }
}

fn run_transformations(ast: &mut Ast, runtime: &mut RuntimeContext, mut data: Data, transforms: &Vec<AstIndex>) -> Result<Data, String> {
    for transform in transforms {
        let transform_cell = ast.get(*transform);
        let transform_ref = &*transform_cell.borrow_mut();

        if let AstElement::Transform(_, transform) = transform_ref {
            // Get Arguments 
            let mut arguments: Vec<Data> = vec![];
            for argument in transform.arguments() {
                let arg = run_expression(ast, runtime, *argument)?;
                arguments.push(arg);
            }

            data = data.apply_transform(transform, arguments);
        }
    }

    Ok(data)
}

/// Get declerations from pattern into a vector of strings
fn run_pattern(ast: &mut Ast, runtime: &mut RuntimeContext, pattern: AstIndex) -> Result<Vec<String>, String> {
    let pat_cell = ast.get(pattern);
    let pat_ref = &mut *pat_cell.borrow_mut();

    let mut declerations: Vec<String> = vec![];
    match pat_ref {
        AstElement::Pattern(_, pat) => {
            match pat {
                Pattern::Decleration(decls) => {
                    for decl in decls {
                        declerations.push(decl.label().unwrap().to_string());
                    }
                }
            }
        },
        _ => return Err("Runtime Error: Let! expected a pattern".to_string()),
    };

    Ok(declerations)
}

// fn match_patterns(ast: &mut Ast, runtime: &mut RuntimeContext, pattern: )