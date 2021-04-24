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
            Expression::Reference(reference) => {
                let _expression = serde.open_tag("ReferenceExpression");
                reference.serialize(serde, ctx)
            }
        }
    }
}

// Transform(function: Token, args: Vec<Expression>)
pub struct Transform(Token);

impl Serializable for Transform {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        let _transform =  serde.open_tag("Transform");
        self.0.serialize(serde, ctx)
    }
}
