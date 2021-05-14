pub mod common;

mod ast;
use ast::prelude::*;

mod data;
pub use data::DataContext;
pub use data::Data;

mod lexer;
use lexer::lexer::Lexer;
use lexer::tokens;

mod parser;
use parser::parser::Parser;

mod semantics;
mod runtime;

pub fn run(input: &str, data: Option<DataContext>) -> String {
    let tokens = run_lexer(input);
    let ast = run_parser(tokens);
    let ast = run_semantics(ast);
    run_runtime(ast, data)
}

pub fn run_lexer(input: &str) -> Vec<tokens::Token> {
    let lexer = Lexer::from(input);
    lexer.tokenize()
}

pub fn run_parser(input: Vec<tokens::Token>) -> Ast {
    let mut parser = Parser::from(input);
    match Parser::parse(&mut parser) {
        Ok(ast) => ast,
        Err(e) => panic!("Parser failed with the error: {:?}", e),
    }
}

pub fn run_semantics(input: Ast) -> Ast {
    match semantics::analyze(input) {
        Ok(ast) => ast,
        Err(e) => panic!("Semantic Analysis failed with the error: {:?}", e),
    }
}

pub fn run_runtime(input: Ast, data: Option<DataContext>) -> String {
    match runtime::run(input, data) {
        Ok(output) => output,
        Err(e) => panic!("Runtime failed with the error: {:?}", e),
    }
}