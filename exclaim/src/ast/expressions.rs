use crate::ast::AstIndex;
use crate::common::serialize::*;
use crate::tokens::Token;

pub enum Expression {
    /// Literal(literal: Token)
    Literal(Token),
    /// Reference(list: Vec<Token>)
    Reference(Vec<Token>)
}

impl Serializable for Expression {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        match self {
            Expression::Literal(literal) => {
                let _expression = serde.open_tag("LiteralExpression");
                literal.serialize(serde, ctx)
            }
            Expression::Reference(reference) => reference.serialize(serde, ctx),
        }
    }
}