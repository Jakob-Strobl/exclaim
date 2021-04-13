use crate::common::serialize::*;
use crate::tokens::Token;
use super::expressions::*;


pub enum Stmt {
    Simple(SimpleStmt),
    Let(LetStmt),
}
impl Serializable for Stmt {
    fn serialize(&self, serde: &mut Serializer) {
        match self {
            Stmt::Simple(stmt) => stmt.serialize(serde),
            Stmt::Let(stmt) => stmt.serialize(serde),
        }
    }
}

pub struct SimpleStmt {
    action: Token,
    expr: Option<Expression>,
}
impl SimpleStmt {
    pub fn new(action: Token, expr: Option<Expression>) -> SimpleStmt {
        SimpleStmt {
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
impl Serializable for SimpleStmt {
    fn serialize(&self, serde: &mut Serializer) {
        let _stmt = serde.open_tag("SimpleStmt");
        {
            let _action = serde.open_tag("action");
            self.action.serialize(serde);
        } // Closes _action tag
        let _expr = serde.open_tag("expr");
        self.expr.serialize(serde);
    }
}

pub struct LetStmt {
    assignee: Token,
    expr: Expression,
}
impl LetStmt {
    pub fn new(assignee: Token, expr: Expression) -> LetStmt {
        LetStmt {
            assignee,
            expr,
        }
    }

    pub fn assignee(&self) -> &Token {
        &self.assignee
    }

    pub fn expr(&self) -> &Expression {
        &self.expr
    }
}
impl Serializable for LetStmt {
    fn serialize(&self, serde: &mut Serializer) {
        let _stmt = serde.open_tag("LetStmt");
        {
            let _assignee = serde.open_tag("assignee");
            self.assignee.serialize(serde);
        } // Closes _action tag
        let _expr = serde.open_tag("expr");
        self.expr.serialize(serde);
    }
}