use std::fmt;
use crate::tokens::Token;

pub trait Node {
    fn token(&self) -> &Token;
    fn next(&self) -> &Option<Box<dyn Node>>;
}

impl fmt::Debug for dyn Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node: [ Token: {:?}, Next: {:?} ]", self.token(), self.next())
    }
}

pub struct TextNode {
    token: Token,
    next: Option<Box<dyn Node>>,
}

impl TextNode {
    pub fn new(token: Token) -> TextNode {
        TextNode {
            token,
            next: None
        }
    }

    pub fn set_next(&mut self, node: Box<dyn Node>) {
        self.next = Some(node);
    }
}

impl Node for TextNode {
    fn token(&self) -> &Token {
        &self.token
    }

    fn next(&self) -> &Option<Box<dyn Node>> {
        &self.next
    }
}

pub struct BlockNode {
    token: Token,
    next: Option<Box<dyn Node>>,
}

impl Node for BlockNode {
    fn token(&self) -> &Token {
        &self.token
    }

    fn next(&self) -> &Option<Box<dyn Node>> {
        &self.next
    }
}