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

        if let Some(head) = ast.head() {
            let mut current_block = head;
            loop {
                let next_block = analyze_block(&mut ast, &mut ctx, current_block)?;

                match next_block {
                    Some(index) => current_block = index,
                    None => break,
                }
            }
        }

        Ok(ast)
    }

    fn analyze_block(ast: &mut Ast, ctx: &mut Context, block_idx: AstIndex) -> SemanticResult<Option<AstIndex>> {
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

                        // Build Scope
                        if let Some(mut current_idx) = *block_next {
                            loop {
                                let next_idx = analyze_block(ast, ctx, current_idx)?;
                                block_scope.push(current_idx);

                                if ctx.scope().was_closed() {
                                    *block_next = next_idx;
                                    break;
                                } else {
                                    match next_idx {
                                        Some(next_idx) => current_idx = next_idx,
                                        None => return Err("Expected the scope to be closed with {{!}}".to_string()),
                                    }
                                }
                            }
                        } else {
                            return Err("Unexpected end of AST when creating a nested scope".to_string());
                        }

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
            _ => return Err("Expected Block!".to_string()),
        }
    }
}