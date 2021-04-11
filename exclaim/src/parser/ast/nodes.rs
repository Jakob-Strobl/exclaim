use crate::tokens::Token;
use crate::common::serialize::*;

use super::expressions::*;

pub enum Node {
    Text(TextNode),
    Block(BlockNode),
    // TODO create seperate file for stmts (stmts.rs)
    Stmt(StmtNode),
}
impl Serializable for Node {
    fn serialize(&self, serde: &mut Serializer) {
        match self {
            Node::Text(text) => text.serialize(serde),
            Node::Block(block) => block.serialize(serde),
            Node::Stmt(stmt) => stmt.serialize(serde)
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

pub struct StmtNode {
    action: Token,
    expr: Option<Expression>,
}
impl StmtNode {
    pub fn new(action: Token, expr: Option<Expression>) -> StmtNode {
        StmtNode {
            action,
            expr,
        }
    }

    pub fn action(&self) -> &Token {
        &self.action
    }

    pub fn expr(&self) -> &Option<Expression> {
        &self.expr
    }
}
impl Serializable for StmtNode {
    fn serialize(&self, serde: &mut Serializer) {
        fn stmt_internals(stmt: &StmtNode, serde: &mut Serializer) {
            Serializer::tag(
                serde,
                "action",
                |serde| stmt.action.serialize(serde)
            );
            
            Serializer::tag(
                serde, 
                "expr",
                |serde| stmt.expr.serialize(serde)
            );
        }

        Serializer::tag(
            serde,
            "StmtNode",
            |serde| stmt_internals(self, serde)
        );
    }
}