pub mod Semantics {
    use crate::semantics::SemanticResult;
    use crate::ast::prelude::Ast;

    pub fn analyze(ast: Ast) -> SemanticResult<Ast> {
        Ok(ast)
    }
}