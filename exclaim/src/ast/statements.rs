use crate::common::serialize::*;
use crate::tokens::Token;

use super::AstIndex;

type ExpressionIndex = AstIndex;
type PatternIndex = AstIndex;

pub enum Statement {
    /// End statement: {{!}}
    /// 
    /// End(action: Token, )
    End(Token),
    /// Let(action: Token, pattern: AstIndex, expression: AstIndex)
    Let(Token, PatternIndex, ExpressionIndex),
    /// Render(action: Token, pattern: AstIndex, expression: AstIndex)
    Render(Token, PatternIndex, ExpressionIndex),
    /// Write(action: Token, expression: AstIndex)
    Write(Token, ExpressionIndex),
}

impl Serializable for Statement {
    fn serialize(&self, serde: &mut Serializer, ctx: &dyn IndexSerializable) -> Option<AstIndex> {
        match self {
            Statement::End(action) => {
                let _statement = serde.open_tag("EndStatement");
                action.serialize(serde, ctx)
            }, 
            Statement::Let(action, pattern, expression) => {
                let _statement = serde.open_tag("LetStatement");
                action.serialize(serde, ctx);
                pattern.serialize(serde, ctx);
                expression.serialize(serde, ctx)
            },
            Statement::Render(action, pattern, expression) => {
                let _statement = serde.open_tag("RenderStatement");
                action.serialize(serde, ctx);
                pattern.serialize(serde, ctx);
                expression.serialize(serde, ctx)
            },
            Statement::Write(action, expression) => {
                let _statement = serde.open_tag("WriteStatement");
                action.serialize(serde, ctx);
                expression.serialize(serde, ctx)
            }
        }
    }
}

