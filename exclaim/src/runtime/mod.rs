use crate::ast::prelude::*;
use crate::data::traits::Renderable;

pub struct Runtime {
    output: String, 
}

impl Runtime {
    fn new() -> Runtime {
        Runtime {
            output: String::new(),
        }
    }

    fn render(&mut self, item: &dyn Renderable) {
        self.output.push_str(&item.render())
    }

    pub fn run(mut ast: Ast) -> Result<String, String> {
        let mut runtime = Runtime::new();

        let mut current_block = ast.head();
        while current_block.is_some() {
            current_block = Runtime::run_block(&mut ast, &mut runtime, current_block)?;
        }

        Ok(runtime.output)
    }

    fn run_block(ast: &mut Ast, runtime: &mut Runtime, block_idx: Option<AstIndex>) -> Result<Option<AstIndex>, String> {
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

    fn run_stmt(ast: &mut Ast, runtime: &mut Runtime, stmt_idx: AstIndex) -> Result<(), String> {
        let element_cell = ast.get(stmt_idx);
        let mut element_ref = element_cell.borrow_mut();

        match &mut *element_ref {
            AstElement::Statement(_, stmt) => {
                match stmt {
                    Statement::Write(_action, expr_idx) => Runtime::run_expr(ast, runtime, *expr_idx),
                    _ => Err("Runtime Error: Stmt Variant Unimplemented".to_string()),
                }
            }
            _ => Err("Runtime Error: Expected a statement".to_string()),
        }
    }

    fn run_expr(ast: &mut Ast, runtime: &mut Runtime, expr_idx: AstIndex) -> Result<(), String> {
        let element_cell = ast.get(expr_idx);
        let mut element_ref = element_cell.borrow_mut();

        match &mut *element_ref {
            AstElement::Expression(_, expr) => {
                match expr {
                    Expression::Literal(literal, _) => {
                        // TODO apply transformations 
                        runtime.render(literal);
                        Ok(())
                    },
                    _ => Err("Runtime Error: Expr Variant Unimplemented".to_string()),
                }
            },
            _ => Err("Runtime Error: Expected an expression".to_string()),
        }
    }
}