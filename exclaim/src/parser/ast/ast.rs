use crate::common::serialize::*;
use super::nodes::*;

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

impl Serializable for Ast {
    fn serialize(&self, serde: &mut Serializer) {
        let _ast = serde.open_tag("Ast");
        for node in self.blocks() {
            node.serialize(serde)
        }
    }
}