use std::{fmt, ops::Deref};
use crate::tokens::Token;

pub enum Node {
    TextNode(TextNode),
    BlockNode(BlockNode),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::TextNode(node) => {
                write!(f, "[ {:?} ] ", node)
            },
            Node::BlockNode(node) => {
                write!(f, "[ {:?} ]", node)
            }
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
        write!(f, "TextNode: {:?}", self.text)
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
        write!(f, "BlockNode: {:?}, {:?}, {:?}", self.open, self.text, self.close)
    }
}