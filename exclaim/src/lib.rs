

pub mod common;

mod ast;
use ast::prelude::*;

mod data;
pub use data::DataContext;
pub use data::Data;

mod tokens;
mod lexer;
mod parser;
mod semantics;
mod runtime;

pub fn run(input: &str, data: Option<DataContext>) -> String {
    let tokens = run_lexer(input);
    let ast = run_parser(tokens);
    let ast = run_semantics(ast);
    run_runtime(ast, data)
}

pub fn run_lexer<S: AsRef<str>>(input: S) -> Vec<tokens::Token> {
    match lexer::run(input) {
        Ok(tokens) => tokens,
        Err(e) => panic!("Lexer failed with the error:\n{:?}", e),
    }
}

pub fn run_parser(input: Vec<tokens::Token>) -> Ast {
    match parser::run(input) {
        Ok(ast) => ast,
        Err(e) => panic!("Parser failed with the error:\n{:?}", e),
    }
}

pub fn run_semantics(input: Ast) -> Ast {
    match semantics::run(input) {
        Ok(ast) => ast,
        Err(e) => panic!("Semantic Analysis failed with the error:\n{:?}", e),
    }
}

pub fn run_runtime(input: Ast, data: Option<DataContext>) -> String {
    match runtime::run(input, data) {
        Ok(output) => output,
        Err(e) => panic!("Runtime failed with the error:\n{:?}", e),
    }
}