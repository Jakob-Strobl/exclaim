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
    literal: Token,
    pipe: Option<PipeSubExpression>
}
impl LiteralExpression {
    pub fn new(literal: Token, pipe: Option<PipeSubExpression>) -> LiteralExpression {
        LiteralExpression {
            literal,
            pipe,
        }
    }

    pub fn literal(&self) -> &Token {
        &self.literal
    }
}
impl Serializable for LiteralExpression {
    fn serialize(&self, serde: &mut Serializer) {
        let _expr = serde.open_tag("LiteralExpression");
        { 
            let _literal = serde.open_tag("literal");
            self.literal.serialize(serde);
        }
        let _pipe = serde.open_tag("pipe");
        self.pipe.serialize(serde);
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

pub struct PipeSubExpression {
    call: Call,
    next: Option<Box<PipeSubExpression>>,
}
impl PipeSubExpression {
    pub fn new(call: Call, next: Option<Box<PipeSubExpression>>) -> PipeSubExpression {
        PipeSubExpression {
            call,
            next,
        }
    }

    pub fn call(&self) -> &Call {
        &self.call
    }

    pub fn next(&self) -> &Option<Box<PipeSubExpression>> {
        &self.next
    }
}
impl Serializable for PipeSubExpression {
    fn serialize(&self, serde: &mut Serializer) {
        let _expr = serde.open_tag("PipeSubExpression"); 
        {
            let _call = serde.open_tag("call");
            self.call.serialize(serde);
        } // Drops _call tag
        let _next = serde.open_tag("next");
        self.next.serialize(serde);
    }
}

pub struct Call {
    function: Token,
    arguments: Option<Arguments>,
}
impl Call {
    pub fn new(function: Token, arguments: Option<Arguments>) -> Call {
        Call {
            function,
            arguments
        }
    }

    pub fn function(&self) -> &Token {
        &self.function
    }

    pub fn arguments(&self) -> &Option<Arguments> {
        &self.arguments
    }
}
impl Serializable for Call {
    fn serialize(&self, serde: &mut Serializer) {
        let _call = serde.open_tag("Call");
        {
            let _function = serde.open_tag("function");
            self.function.serialize(serde);
        } // Drops _fn_name tag
        let _args = serde.open_tag("arguments");
        self.arguments.serialize(serde);
    }
}

pub struct Arguments {
    args: Vec<Expression>
}
impl Arguments {
    pub fn new(args: Vec<Expression>) -> Arguments {
        Arguments {
            args,
        }
    }

    pub fn args(&self) -> &Vec<Expression> {
        &self.args
    }
}
impl Serializable for Arguments {
    fn serialize(&self, serde: &mut Serializer) {
        let _args = serde.open_tag("Arguments");
        for arg in &self.args {
            let _arg = serde.open_tag("arg");
            arg.serialize(serde)
        }
    }
}
