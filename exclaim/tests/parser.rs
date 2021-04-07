use std::fmt::format;

use exclaim::*;

#[test]
pub fn parser_test_empty_input() {
    let expected = "AST:\n";
    let input = "";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, format!("{:?}", ast));
}

#[test]
pub fn parser_test_string_literal() {
    let expected = "AST:\nNode: [ Token: Token { kind: StringLiteral, lexeme: \"Hello, World!\", location: Location { line: 0, column: 0 } } ]\n";
    let input = "Hello, World!";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, format!("{:?}", ast));
}

#[test]
pub fn parser_test_simple_write_block() {
    let input = "{{ \"Hello!\" }}";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    println!("{:?}", ast);
}