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