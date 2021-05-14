use std::fmt;

const FILE_SCOPE: usize = 0;
struct Scope {
    level: usize, 
    was_closed: bool,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            level: FILE_SCOPE,
            was_closed: false,
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn open(&mut self) {
        println!("Opened scope!");
        self.level += 1;
    }

    pub fn close(&mut self) {
        println!("Closed scope!");
        self.level -= 1;
        self.was_closed = true;
    }

    pub fn was_closed(&mut self) -> bool {
        if self.was_closed {
            self.was_closed = false;
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.level == FILE_SCOPE {
            write!(f, "File Scope")
        } else {
            write!(f, "Local Scope ({})", self.level)
        }
    }
}

struct Context {
    scope: Scope,
}

impl Context {
    pub fn new() -> Context {
        Context {
            scope: Scope::new()
        }
    }

    pub fn scope(&mut self) -> &mut Scope {
        &mut self.scope
    }
}

pub mod Semantics {
    use crate::semantics::SemanticResult;
    use crate::ast::prelude::*;

    use super::*;

    macro_rules! unwrap_element {
        ( $obj:ident.$fn:ident($idx:ident) ) => {
            match $obj.$fn($idx) {
                Some(element) => element,
                None => return Err(format!("Expected to recieve an AST Element at index {:?}", $idx)),
            };
        };
    }

    pub fn analyze(mut ast: Ast) -> SemanticResult<Ast> {
        let mut ctx = Context::new();

        let mut current_block = ast.head();
        while current_block.is_some() {
            current_block = analyze_block(&mut ast, &mut ctx, current_block)?;
        }

        Ok(ast)
    }

    fn analyze_block(ast: &mut Ast, ctx: &mut Context, block_idx: Option<AstIndex>) -> SemanticResult<Option<AstIndex>> {
        if let Some(block_idx) = block_idx {
            let element_cell = ast.get(block_idx);
            
            // Check element is a block
            let mut element_ref = element_cell.borrow_mut();
            match &mut *element_ref {
                AstElement::Block(_, block) => { 
                    println!("Block.text: {:?}", block.text());
                    println!("Block.stmt: {:?}", block.stmt());
                    
                    match block {
                        // Text Blocks can't fail in this context, because they are just text
                        Block::Text(_, next) => Ok(*next),
                        Block::CodeEnclosed(_, next) => {
                            // TODO analyze the statement
                            Ok(*next) 
                        }
                        Block::CodeUnclosed(_, block_scope, block_next) => { 
                            // TODO analyze the statement
                            
                            // Open Scope 
                            ctx.scope().open();

                            // Build the scope until it is closed
                            let mut current_idx = *block_next;
                            while !ctx.scope().was_closed() {
                                let next_idx = match analyze_block(ast, ctx, current_idx) {
                                    Ok(idx) => idx,
                                    Err(_) => return Err("Expected the scope to be closed with {{!}}".to_string()),
                                };

                                // analyze_block() would return an error if current_idx is None
                                // so, we know we can unwrap current_idx
                                block_scope.push(current_idx.unwrap());
                                current_idx = next_idx;
                            }
                            *block_next = current_idx;

                            Ok(*block_next)
                        }
                        Block::CodeClosing(_, next) => {
                            if ctx.scope().level() == FILE_SCOPE {
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
        } else {
            return Err("Unexpected end of AST. Expected a block.".to_string());
        }
    }
}