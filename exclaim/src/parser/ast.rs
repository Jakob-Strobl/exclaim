use std::fmt;

use super::node::*;
pub struct Ast {
    blocks: Vec<Node>,
}

impl Ast {
    pub fn new() -> Ast {
        Ast {
            blocks: vec![]
        }
    }

    pub fn blocks(&self) -> &Vec<Node> {
        &self.blocks
    }
}

impl Ast {
    pub fn push_block(&mut self, node: Node) {
        self.blocks.push(node);
    }
}

impl fmt::Debug for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!("AST:\n");
        for block in &self.blocks {
            output += &format!("{:?}\n", block);
        }

        f.write_str(&output)
    }
}