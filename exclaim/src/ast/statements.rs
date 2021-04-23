use crate::common::serialize::*;
use crate::tokens::Token;

use super::AstIndex;

pub enum Statement {
    /// End statement: {{!}}
    /// 
    /// End(action)
    End(Token)
}

impl Serializable for Statement {
    fn serialize(&self, serde: &mut Serializer) -> &Option<AstIndex> {
        match self {
            Statement::End(action) => {
                let _statement = serde.open_tag("EndStatement");
                action.serialize(serde);
                &None 
            }
        }
    }
}

