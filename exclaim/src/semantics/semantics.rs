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

    fn analyze_block(ast: &mut Ast, block: AstIndex) -> SemanticResult<Option<AstIndex>> {
        let element = unwrap_element!(ast.get_mut(block));
        
        match element {
            AstElement::Block(_, block) => {
                println!("Block.text: {:?}", block.text());
                println!("Block.stmt: {:?}", block.stmt());
                match block {
                    // Text Blocks can't fail in this context, because they are just text
                    Block::Text(_, next) => Ok(*next), // This works because by dereferencing we implicitly copy the AstIndex
                    Block::CodeEnclosed(_, next) => {
                        // TODO analyze the contents of the block 
                        Ok(*next) 
                    }
                    Block::CodeUnclosed(_, _, next) => { 
                        // TODO analyze the contents of the block 
                        Ok(*next) 
                    }
                    Block::CodeClosing(_, _) => {
                        // A closing block should not exist by itself in the file scope
                        Err("Invalid lone closing block in file scope. Closing blocks must only be used to close an opened scope.".to_string())
                    }
                }
            }
            _ => return Err("Expected Block!".to_string()),
        }
    }
}