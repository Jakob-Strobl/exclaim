use crate::ast::AstIndex;
use crate::common::serialize::*;
use crate::tokens::Token;

type TransformIndex = AstIndex;
type ExpressionIndex = AstIndex;

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

// Transform(label: Token, args: Vec<Expression>)
pub struct Transform(Token, Vec<ExpressionIndex>);

impl Transform {
    pub fn new(label: Token, arguments: Vec<ExpressionIndex>) -> Transform {
        Transform(label, arguments)
    }
}

impl Serializable for Transform {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        let _transform =  serde.open_tag("Transform");
        self.0.serialize(serde, ctx);

        let _arguments = serde.open_tag("Arguments");
        self.1.serialize(serde, ctx)
    }
}
