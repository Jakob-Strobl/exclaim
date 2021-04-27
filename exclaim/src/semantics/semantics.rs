pub mod Semantics {
    use crate::semantics::SemanticResult;
    use crate::ast::prelude::*;

    macro_rules! unwrap_element {
        ( $obj:ident.$fn:ident($idx:ident) ) => {
            match $obj.$fn($idx) {
                Some(element) => element,
                None => return Err(format!("Expected to recieve an AST Element at index {:?}", $idx)),
            };
        };
    }

    pub fn analyze(mut ast: Ast) -> SemanticResult<Ast> {
        if let Some(head) = ast.head() {
            let mut current_block = head;
            loop {
                let next_block = analyze_block(&mut ast, current_block)?;

                match next_block {
                    Some(index) => current_block = index,
                    None => break,
                }
            }
        }

        Ok(ast)
    }

    fn analyze_block(ast: &mut Ast, block_idx: AstIndex) -> SemanticResult<Option<AstIndex>> {
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
                        // TODO analyze the contents of the block 
                        Ok(*next) 
                    }
                    Block::CodeUnclosed(_, scope, next) => { 
                        // TODO analyze the contents of the block 

                        // Build Scope of Unclosed block
                        if let Some(next_block) = next {
                            let mut current_block = *next_block;
                            loop {
                                let element_cell = ast.get(current_block);
                                let mut element_ref = element_cell.borrow_mut();
                                match &mut *element_ref {
                                    AstElement::Block(self_idx, block) => {
                                        // TODO ? Analyze contents of block?

                                        // Push the current block into the scope of the CodeUclosed block
                                        scope.push(*self_idx);

                                        match block {
                                            // If code closing, set the next value of the Unclosed block to the closing block's next
                                            Block::CodeClosing(_, scoped_next) => {
                                                *next = *scoped_next;
                                                break; // Break from scope
                                            },
                                            _ => {
                                                // If not CodeClosing, set the index of current_block to the next block
                                                current_block = match block.next() {
                                                    Some(index) => *index,
                                                    None => return Err("Unexpected end of AST: Expected the scope to be closed with {{!}}".to_string())
                                                };
                                            }
                                        }
                                    },
                                    _ => return Err("Expected a Block.".to_string())
                                };
                            }
                        } else {
                            // next must exist, since unclosed blocks must be closed
                            return Err("Unclosed block's next is None. An unclosed block is expected to have a closing block".to_string());
                        }
                        Ok(*next)
                    }
                    Block::CodeClosing(_, _) => {
                        // A closing block should not exist by itself in the file scope
                        return Err("Invalid lone closing block in file scope. Closing blocks must only be used to close an opened scope.".to_string());
                    }
                }
            }
            _ => return Err("Expected Block!".to_string()),
        }
    }
}