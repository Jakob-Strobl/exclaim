use crate::tokens::Token;
use crate::Serializeable;
use crate::AstSerializer;

use super::expressions::*;

pub enum Node {
    Text(TextNode),
    Block(BlockNode),
    Stmt(StmtNode),
}
impl Serializeable for Node {
    fn serialize(&self, serde: &mut AstSerializer) {
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
impl Serializeable for TextNode {
    fn serialize(&self, serde: &mut AstSerializer) {
        fn text_internals(text: &TextNode, serde: &mut AstSerializer) {
            AstSerializer::tag(
                serde,
                "text", 
                |serde| text.text().serialize(serde)
            );
        }

        AstSerializer::tag(
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
impl Serializeable for BlockNode {
    fn serialize(&self, serde: &mut AstSerializer) {
        fn block_internals(block: &BlockNode, serde: &mut AstSerializer) {
            AstSerializer::tag(
                serde, 
                "stmt",
                |serde| block.stmt.serialize(serde)
            );
        }

        AstSerializer::tag(
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
impl Serializeable for StmtNode {
    fn serialize(&self, serde: &mut AstSerializer) {
        fn stmt_internals(stmt: &StmtNode, serde: &mut AstSerializer) {
            AstSerializer::tag(
                serde,
                "action",
                |serde| stmt.action.serialize(serde)
            );
            
            AstSerializer::tag(
                serde, 
                "expr",
                |serde| stmt.expr.serialize(serde)
            );
        }

        AstSerializer::tag(
            serde,
            "StmtNode",
            |serde| stmt_internals(self, serde)
        );
    }
}