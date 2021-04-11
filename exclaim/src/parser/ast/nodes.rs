use crate::tokens::Token;
use crate::common::serialize::*;

use super::statements::StmtNode;

pub enum Node {
    Text(TextNode),
    Block(BlockNode),
}
impl Serializable for Node {
    fn serialize(&self, serde: &mut Serializer) {
        match self {
            Node::Text(text) => text.serialize(serde),
            Node::Block(block) => block.serialize(serde),
        }
    }
}

pub struct TextNode {
    text: Token,
}
impl TextNode {
    pub fn new(text: Token) -> TextNode {
        TextNode {
            text
        }
    }

    pub fn text(&self) -> &Token {
        &self.text
    }
}
impl Serializable for TextNode {
    fn serialize(&self, serde: &mut Serializer) {
        fn text_internals(text: &TextNode, serde: &mut Serializer) {
            Serializer::tag(
                serde,
                "text", 
                |serde| text.text().serialize(serde)
            );
        }

        Serializer::tag(
            serde, 
            "TextNode",
            |serde| text_internals(self, serde)
        );
    }
}

pub struct BlockNode {
    stmt: StmtNode,
}
impl BlockNode {
    pub fn new(stmt: StmtNode) -> BlockNode {
        BlockNode {
            stmt,
        }
    }

    pub fn stmt(&self) -> &StmtNode {
        &self.stmt
    }
}
impl Serializable for BlockNode {
    fn serialize(&self, serde: &mut Serializer) {
        fn block_internals(block: &BlockNode, serde: &mut Serializer) {
            Serializer::tag(
                serde, 
                "stmt",
                |serde| block.stmt.serialize(serde)
            );
        }

        Serializer::tag(
            serde, 
            "BlockNode",
            |serde| block_internals(self, serde)
        );
    }
}