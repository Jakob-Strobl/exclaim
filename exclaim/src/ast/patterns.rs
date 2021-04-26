use crate::common::serialize::*;
use crate::tokens::Token;
pub enum Pattern {
    // Unit(decl: Token(Label))
    Unit(Token),
}

impl Serializable for Pattern {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<super::AstIndex> {
        match self {
            Pattern::Unit(decl) => {
                let _pattern = serde.open_tag("UnitPattern");
                decl.serialize(serde, ctx)
            }
        }
    }
}

