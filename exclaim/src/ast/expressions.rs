use crate::ast::AstIndex;
use crate::common::serialize::*;
use crate::tokens::Token;

type TransformIndex = AstIndex;

pub enum Expression {
    /// Literal(literal: Token)
    Literal(Token, Option<Vec<TransformIndex>>),
    /// Reference(list: Vec<Token>)
    Reference(Vec<Token>, Option<Vec<TransformIndex>>)
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

// Transform(label: Token, args: Vec<Expression>)
pub struct Transform(pub Token);

impl Serializable for Transform {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        let _transform =  serde.open_tag("Transform");
        self.0.serialize(serde, ctx)
    }
}
