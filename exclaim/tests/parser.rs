use std::fmt::format;

use exclaim::*;

#[test]
#[should_panic(expected = "Kind(\"unexpected end of token stream\")")]
pub fn parser_test_empty_input() {
    let input = "";

    let tokens = crate::run_lexer(input);
    let _ = crate::run_parser(tokens);
}

#[test]
pub fn parser_test_string_literal() {
    let expected = "AST:\nNode: [ Token: Token { kind: StringLiteral, lexeme: \"Hello, World!\", location: Location { line: 0, column: 0 } }, Next: None ]";
    let input = "Hello, World!";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, format!("{:?}", ast));
}