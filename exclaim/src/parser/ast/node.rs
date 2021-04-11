use crate::tokens::Token;
use crate::Serializeable;
use crate::AstSerializer;

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
            Node::Stmt(stmt) => ()
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

pub enum Expression {
    Literal(LiteralExpression),
    Reference(ReferenceExpression),
}
impl Serializeable for Expression {
    fn serialize(&self, serde: &mut AstSerializer) {
        match self {
            Expression::Literal(literal) => literal.serialize(serde),
            Expression::Reference(reference) => reference.serialize(serde),
        }
    }
}

pub struct LiteralExpression {
    literal: Token
}
impl LiteralExpression {
    pub fn new(literal: Token) -> LiteralExpression {
        LiteralExpression {
            literal,
        }
    }

    pub fn literal(&self) -> &Token {
        &self.literal
    }
}
impl Serializeable for LiteralExpression {
    fn serialize(&self, serde: &mut AstSerializer) {
        fn literal_internals(expr: &LiteralExpression, serde: &mut AstSerializer) {
            AstSerializer::tag(
                serde,
                "literal",
                |serde| expr.literal.serialize(serde)
            );
        }
        
        AstSerializer::tag(
            serde, 
            "LiteralExpression",
            |serde| literal_internals(self, serde)
        );
    }
}

pub struct ReferenceExpression {
    reference: Token,
    child: Option<Box<ReferenceExpression>>,
}
impl ReferenceExpression {
    pub fn new(reference: Token, child: Option<Box<ReferenceExpression>>) -> ReferenceExpression {
        ReferenceExpression {
            reference,
            child,
        }
    }

    pub fn reference(&self) -> &Token {
        &self.reference
    }

    pub fn child(&self) -> &Option<Box<ReferenceExpression>> {
        &self.child
    }
}
impl Serializeable for ReferenceExpression {
    fn serialize(&self, serde: &mut AstSerializer) {
        fn reference_internals(expr: &ReferenceExpression, serde: &mut AstSerializer) {
            AstSerializer::tag(
                serde,
                "reference",
                |serde| expr.reference.serialize(serde)
            );

            AstSerializer::tag(
                serde, 
                "child",
                |serde| expr.child.serialize(serde)
            );
        }

        AstSerializer::tag(
            serde,
            "ReferenceExpression",
            |serde| reference_internals(self, serde)
        );
    }
}

