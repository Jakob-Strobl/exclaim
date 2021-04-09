use std::fmt;
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

    pub fn text(&self) -> &Token {
        &self.text
    }
}
impl fmt::Debug for TextNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ TextNode: text: {:?} ]", self.text)
    }
}

pub struct BlockNode {
    stmt: Option<StmtNode>,
}
impl BlockNode {
    pub fn new() -> BlockNode {
        BlockNode {
            stmt: None,
        }
    }

    pub fn set_stmt(&mut self, stmt: StmtNode) {
        self.stmt = Some(stmt);
    }

    pub fn stmt(&self) -> &Option<StmtNode> {
        &self.stmt
    }
}
impl fmt::Debug for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ BlockNode: stmt: {:?}, ]", self.stmt)
    }
}

pub struct StmtNode {
    action: Token,
    expr: Option<Expression>,
}
impl StmtNode {
    pub fn new(action: Token) -> StmtNode {
        StmtNode {
            action,
            expr: None,
        }
    }

    pub fn set_expr(&mut self, expr: Expression) {
        self.expr = Some(expr);
    }

    pub fn action(&self) -> &Token {
        &self.action
    }

    pub fn expr(&self) -> &Option<Expression> {
        &self.expr
    }
}
impl fmt::Debug for StmtNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ StmtNode: action: {:?}, expr: {:?} ]", self.action, self.expr)
    }
}


pub enum Expression {
    Literal(LiteralExpression)
}
impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(literal) => write!(f, "{:?}", literal),
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
impl fmt::Debug for LiteralExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ LiteralExpr: literal, expr: {:?} ]", self.literal)
    }
}