use crate::common::serialize::*;
use crate::tokens::Token;
use super::expressions::*;
use super::patterns::*;


pub enum Statement {
    Simple(SimpleStatement),
    Let(LetStatement),
}
impl Serializable for Statement {
    fn serialize(&self, serde: &mut Serializer) {
        match self {
            Statement::Simple(stmt) => stmt.serialize(serde),
            Statement::Let(stmt) => stmt.serialize(serde),
        }
    }
}

pub struct SimpleStatement {
    action: Token,
    expr: Option<Expression>,
}
impl SimpleStatement {
    pub fn new(action: Token, expr: Option<Expression>) -> SimpleStatement {
        SimpleStatement {
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
impl Serializable for SimpleStatement {
    fn serialize(&self, serde: &mut Serializer) {
        let _stmt = serde.open_tag("SimpleStatement");
        {
            let _action = serde.open_tag("action");
            self.action.serialize(serde);
        } // Closes _action tag
        let _expr = serde.open_tag("expr");
        self.expr.serialize(serde);
    }
}

pub struct LetStatement {
    assignee: Pattern,
    expr: Expression,
}
impl LetStatement {
    pub fn new(assignee: Pattern, expr: Expression) -> LetStatement {
        LetStatement {
            assignee,
            expr,
        }
    }

    pub fn assignee(&self) -> &Pattern {
        &self.assignee
    }

    pub fn expr(&self) -> &Expression {
        &self.expr
    }
}
impl Serializable for LetStatement {
    fn serialize(&self, serde: &mut Serializer) {
        let _stmt = serde.open_tag("LetStatement");
        {
            let _assignee = serde.open_tag("assignee");
            self.assignee.serialize(serde);
        } // Closes _action tag
        let _expr = serde.open_tag("expr");
        self.expr.serialize(serde);
    }
}