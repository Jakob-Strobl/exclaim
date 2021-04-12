use crate::tokens::Token;
use crate::common::serialize::*;

pub enum Expression {
    Literal(LiteralExpression),
    Reference(ReferenceExpression),
}
impl Serializable for Expression {
    fn serialize(&self, serde: &mut Serializer) {
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
impl Serializable for LiteralExpression {
    fn serialize(&self, serde: &mut Serializer) {
        let _expr = serde.open_tag("LiteralExpression");
        let _literal = serde.open_tag("literal");
        self.literal.serialize(serde);
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
impl Serializable for ReferenceExpression {
    fn serialize(&self, serde: &mut Serializer) {
        let _expr = serde.open_tag("ReferenceExpression"); 
        {
            let _reference = serde.open_tag("reference");
            self.reference.serialize(serde);
        } // Closes _reference tag
        let _child = serde.open_tag("child");
        self.child.serialize(serde);
    }
}