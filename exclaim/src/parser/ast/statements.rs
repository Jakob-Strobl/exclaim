use crate::common::serialize::*;
use crate::tokens::Token;
use super::expressions::Expression;

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
        let _stmt = serde.open_tag("StmtNode");
        {
            let _action = serde.open_tag("action");
            self.action.serialize(serde);
        } // Closes _action tag
        let _expr = serde.open_tag("expr");
        self.expr.serialize(serde);
    }
}