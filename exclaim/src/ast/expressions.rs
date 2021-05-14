use crate::ast::AstIndex;
use crate::common::serialize::*;
use crate::tokens::Token;

type TransformIndex = AstIndex;

pub enum Expression {
    /// Literal(literal: Token)
    Literal(Token, Vec<TransformIndex>),
    /// Reference(list: Vec<Token>)
    Reference(Vec<Token>, Vec<TransformIndex>)
}

impl Serializable for Expression {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        match self {
            Expression::Literal(literal, transforms) => {
                let _expression = serde.open_tag("LiteralExpression");
                literal.serialize(serde, ctx);

                let _tranforms = serde.open_tag("Transforms");
                transforms.serialize(serde, ctx)
            }
            Expression::Reference(reference, transforms) => {
                let _expression = serde.open_tag("ReferenceExpression");
                reference.serialize(serde, ctx);

                let _tranforms = serde.open_tag("Transforms");
                transforms.serialize(serde, ctx)
            }
        }
    }
}