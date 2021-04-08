use std::{fmt::{self, write}, ops::Deref};
use crate::tokens::Token;

pub enum Node {
    Text(TextNode),
    Block(BlockNode),
    Stmt(StmtNode),
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Text(node) => write!(f, "{:?}", node),
            Node::Block(node) => write!(f, "{:?}", node),
            Node::Stmt(node) => write!(f, "{:?}", node),
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
}
impl fmt::Debug for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ TextNode: text: {:?} ]", self.text)
    }
}

pub struct BlockNode {
    open: Token,
    text: Option<TextNode>,
    close: Option<Token>,
}
impl BlockNode {
    pub fn new(open: Token) -> BlockNode {
        BlockNode {
            open,
            text: None,
            close: None,
        }
    }

    pub fn set_text(&mut self, text: TextNode) {
        self.text = Some(text);
    }

    pub fn set_close(&mut self, close: Token) {
        self.close = Some(close);
    }
}
impl fmt::Debug for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ BlockNode: open: {:?}, text: {:?}, close: {:?} ]", self.open, self.text, self.close)
    }
}