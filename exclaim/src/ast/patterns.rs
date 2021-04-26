use crate::common::serialize::*;
use crate::tokens::Token;
pub enum Pattern {
    // Decleration(decls: Vec<Token>)
    Decleration(Vec<Token>)
}

impl Serializable for Pattern {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<super::AstIndex> {
        match self {
            Pattern::Decleration(decls) => {
                let _pattern = serde.open_tag("DeclerationPattern");
                decls.serialize(serde, ctx)
            },
        }
    }
}

