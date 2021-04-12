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
        let _text_node_tag = serde.open_tag("TextNode");
        let _text_tag = serde.open_tag("text");
        self.text().serialize(serde);
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
        let _block_node = serde.open_tag("BlockNode");
        let _block_stmt = serde.open_tag("stmt");
        self.stmt.serialize(serde)
    }
}
