use crate::tokens::Token;
use crate::common::serialize::*;

use super::statements::Stmt;

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
        let _node = serde.open_tag("TextNode");
        let _text = serde.open_tag("text");
        self.text().serialize(serde);
    }
}

pub struct BlockNode {
    stmt: Stmt,
}
impl BlockNode {
    pub fn new(stmt: Stmt) -> BlockNode {
        BlockNode {
            stmt,
        }
    }

    pub fn stmt(&self) -> &Stmt {
        &self.stmt
    }
}
impl Serializable for BlockNode {
    fn serialize(&self, serde: &mut Serializer) {
        let _node = serde.open_tag("BlockNode");
        let _stmt = serde.open_tag("stmt");
        self.stmt.serialize(serde)
    }
}
