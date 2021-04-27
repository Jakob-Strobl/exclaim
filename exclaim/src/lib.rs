pub mod common;

mod ast;
use ast::prelude::*;

mod lexer;
use lexer::lexer::Lexer;
use lexer::tokens;

mod parser;
use parser::parser::Parser;

mod semantics;
use semantics::semantics::Semantics;

pub fn run(input: &str) {
    let tokens = run_lexer(input);
    let ast = run_parser(tokens);
    let _ast = run_semantics(ast);
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
    match Semantics::analyze(input) {
        Ok(ast) => ast,
        Err(e) => panic!("Semantic Analysis failed with the error: {:?}", e),
    }
}