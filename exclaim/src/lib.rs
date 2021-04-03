mod util;

mod lexer;
use lexer::lexer::Lexer;
use lexer::tokens;

mod parser;
use parser::parser::Parser;

pub fn run(input: &str) {
    let tokens = run_lexer(input);
    let ast = run_parser(tokens);
}

pub fn run_lexer(input: &str) -> Vec<tokens::Token> {
    let lexer = Lexer::from(input);
    lexer.tokenize()
}

pub fn run_parser(input: Vec<tokens::Token>) {
    let parser = Parser::from(input);
    parser.parse()
}