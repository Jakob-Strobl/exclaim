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
    open: Token,
    stmt: Option<StmtNode>,
    close: Option<Token>,
}
impl BlockNode {
    pub fn new(open: Token) -> BlockNode {
        BlockNode {
            open,
            stmt: None,
            close: None,
        }
    }

    pub fn set_stmt(&mut self, stmt: StmtNode) {
        self.stmt = Some(stmt);
    }

    pub fn set_close(&mut self, close: Token) {
        self.close = Some(close);
    }
}
impl fmt::Debug for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ BlockNode: open: {:?}, stmt: {:?}, close: {:?} ]", self.open, self.stmt, self.close)
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
}
impl fmt::Debug for LiteralExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ LiteralExpr: literal, expr: {:?} ]", self.literal)
    }
}