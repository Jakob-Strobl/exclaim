use crate::common::serialize::*;
use crate::tokens::Token;

use super::AstIndex;

type ExpressionIndex = AstIndex;

pub enum Statement {
    /// End statement: {{!}}
    /// 
    /// End(action: Token, )
    End(Token),

    /// Write(action: Token, expr_idx: ExpressionIndex)
    Write(Token, ExpressionIndex)
}

impl Serializable for Statement {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        match self {
            Statement::End(action) => {
                let _statement = serde.open_tag("EndStatement");
                action.serialize(serde, ctx)
            }, 
            Statement::Write(action, expr_idx) => {
                let _statement = serde.open_tag("WriteStatement");
                action.serialize(serde, ctx);
                
                let expression = ctx.get(expr_idx).unwrap();
                expression.serialize(serde, ctx)
            }
        }
    }
}

