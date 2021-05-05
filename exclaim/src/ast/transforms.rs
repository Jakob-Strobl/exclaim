use crate::tokens::Token;
use crate::common::serialize::*;

use super::AstIndex;

type ExpressionIndex = AstIndex;

// Transform(label: Token, args: Vec<AstIndex>)
#[derive(Debug)]
pub struct Transform(Token, Vec<ExpressionIndex>);

impl Transform {
    pub fn new(label: Token, arguments: Vec<ExpressionIndex>) -> Transform {
        Transform(label, arguments)
    }

    pub fn signature(&self) -> (&str, usize) {
        (self.name(), self.1.len())
    }

    pub fn has_arguments(&self) -> bool {
        self.1.len() != 0
    }

    pub fn name(&self) -> &str {
        match self.0.label() {
            Some(name) => name,
            None => panic!("Expected transform to have a name; token is not a label.")
        }
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
