use crate::tokens::Token;
use crate::Serializeable;
use crate::AstSerializer;

pub enum Expression {
    Literal(LiteralExpression),
    Reference(ReferenceExpression),
}
impl Serializeable for Expression {
    fn serialize(&self, serde: &mut AstSerializer) {
        match self {
            Expression::Literal(literal) => literal.serialize(serde),
            Expression::Reference(reference) => reference.serialize(serde),
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

    pub fn literal(&self) -> &Token {
        &self.literal
    }
}
impl Serializeable for LiteralExpression {
    fn serialize(&self, serde: &mut AstSerializer) {
        fn literal_internals(expr: &LiteralExpression, serde: &mut AstSerializer) {
            AstSerializer::tag(
                serde,
                "literal",
                |serde| expr.literal.serialize(serde)
            );
        }
        
        AstSerializer::tag(
            serde, 
            "LiteralExpression",
            |serde| literal_internals(self, serde)
        );
    }
}

pub struct ReferenceExpression {
    reference: Token,
    child: Option<Box<ReferenceExpression>>,
}
impl ReferenceExpression {
    pub fn new(reference: Token, child: Option<Box<ReferenceExpression>>) -> ReferenceExpression {
        ReferenceExpression {
            reference,
            child,
        }
    }

    pub fn reference(&self) -> &Token {
        &self.reference
    }

    pub fn child(&self) -> &Option<Box<ReferenceExpression>> {
        &self.child
    }
}
impl Serializeable for ReferenceExpression {
    fn serialize(&self, serde: &mut AstSerializer) {
        fn reference_internals(expr: &ReferenceExpression, serde: &mut AstSerializer) {
            AstSerializer::tag(
                serde,
                "reference",
                |serde| expr.reference.serialize(serde)
            );

            AstSerializer::tag(
                serde, 
                "child",
                |serde| expr.child.serialize(serde)
            );
        }

        AstSerializer::tag(
            serde,
            "ReferenceExpression",
            |serde| reference_internals(self, serde)
        );
    }
}