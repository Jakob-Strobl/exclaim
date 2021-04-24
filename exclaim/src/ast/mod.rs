use std::ops::Deref;
// Using a concrete type so one is not accidentally using indexes from normal math out of thin air 
#[derive(Debug, Clone, Copy)]
pub struct AstIndex(usize);

impl Deref for AstIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub mod ast;
pub mod blocks;
pub mod statements;
pub mod expressions;

pub mod prelude {
    pub use super::AstIndex;
    pub use super::ast::Ast;
    pub use super::ast::Pushable;
    pub use super::ast::AstElement;
    pub use super::blocks::Block;
    pub use super::statements::Statement;
    pub use super::expressions::Expression;
    pub use super::expressions::Transform;
}