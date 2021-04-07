use std::fmt;
use crate::tokens::Token;

pub trait Node {
    fn token(&self) -> &Token;
    // fn next(&self) -> &Option<Box<dyn Node>>;
}

impl fmt::Debug for dyn Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node: [ Token: {:?} ]", self.token())
    }
}

pub struct TextNode {
    token: Token,
}

impl Node for TextNode {
    fn token(&self) -> &Token {
        &self.token
    }

    // fn next(&self) -> &Option<Box<dyn Node>> {
    //     &self.next
    // }
}

impl TextNode {
    pub fn new(token: Token) -> TextNode {
        TextNode {
            token,
        }
    }
}

pub struct BlockNode {
    token: Token,
    stmt: Option<Box<dyn Node>>,
}


impl Node for BlockNode {
    fn token(&self) -> &Token {
        &self.token
    }
}

impl BlockNode {
    pub fn new(token: Token) -> BlockNode {
        BlockNode {
            token,
            stmt: None
        }
    }

    pub fn set_stmt(&mut self, node: Box<dyn Node>) {
        self.stmt = Some(node);
    }
}