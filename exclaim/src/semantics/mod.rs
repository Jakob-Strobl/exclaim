use crate::ast::prelude::*;

pub mod scope;
use scope::Scope;

type SemanticResult<T> = Result<T, String>;

struct SemanticContext {
    scope: Scope,
}

impl SemanticContext {
    pub fn new() -> SemanticContext {
        SemanticContext {
            scope: Scope::new()
        }
    }

    pub fn scope(&mut self) -> &mut Scope {
        &mut self.scope
    }
}

macro_rules! unwrap_index {
    ($index:expr) => {
        if let Some(value) = $index {
            value
        } else {
            return Err("Expected an AST Index.".to_string());
        }
    };
}

pub fn run(ast: Ast) -> SemanticResult<Ast> {
    analyze(ast)
}

fn analyze(mut ast: Ast) -> SemanticResult<Ast> {
    let mut ctx = SemanticContext::new();

    let mut current_block = ast.head();
    while current_block.is_some() {
        current_block = analyze_block(&mut ast, &mut ctx, current_block)?;
    }

    Ok(ast)
}

fn analyze_block(ast: &mut Ast, ctx: &mut SemanticContext, block: Option<AstIndex>) -> SemanticResult<Option<AstIndex>> {
    let block = unwrap_index!(block);
    let block_cell = ast.get(block);
    let mut block_ref = block_cell.borrow_mut();
    
    // Check element is a block
    match &mut *block_ref {
        AstElement::Block(_, block) => { 
            match block {
                // Text Blocks can't fail in this context, because they are just text
                Block::Text(_, next) => Ok(*next),
                Block::CodeEnclosed(_, next) => {
                    // TODO analyze the statement - Not necessary for now
                    Ok(*next) 
                }
                Block::CodeUnclosed(_, block_scope, next_block) => { 
                    // TODO analyze the statement - Not necessary for now
                    
                    // Open Scope 
                    ctx.scope().open();

                    // Build the scope until it is closed
                    let mut current_scoped_block = *next_block;
                    while !ctx.scope().was_closed() {
                        let next_scoped_block = match analyze_block(ast, ctx, current_scoped_block) {
                            Ok(index) => index,
                            Err(_) => return Err("Expected the scope to be closed with {{!}}".to_string()),
                        };

                        // analyze_block() would return an error if current_idx is None
                        // so, we know we can unwrap current_idx
                        block_scope.push(current_scoped_block.unwrap());
                        current_scoped_block = next_scoped_block;
                    }
                    *next_block = current_scoped_block;

                    Ok(*next_block)
                }
                Block::CodeClosing(_, next) => {
                    if ctx.scope().level() == scope::FILE_SCOPE {
                        // A closing block should not exist by itself in the file scope
                        return Err("Invalid lone closing block in file scope. Closing blocks must only be used to close an opened scope.".to_string());
                    } else {
                        ctx.scope().close();
                        Ok(*next)
                    }
                }
            }
        }
        _ => return Err("Expected a Block!".to_string()),
    }
}