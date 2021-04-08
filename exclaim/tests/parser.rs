use exclaim::*;

#[test]
pub fn parse_empty_input() {
    let expected = "AST:\n";
    let input = "";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, format!("{:?}", ast));
}

#[test]
pub fn parse_string_literal() {
    let expected = "AST:\n[ TextNode: text: Token { kind: StringLiteral, lexeme: \"Hello, World!\", location: Location { line: 0, column: 0 } } ]\n";
    let input = "Hello, World!";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, format!("{:?}", ast));
}

#[test]
pub fn parse_write_string() {
    let input = "{{ write! \"Hello!\" }}";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    println!("{:?}", ast);
}

#[test]
pub fn parse_simple() {
    let input = "This is a string. {{ write! \"Hello!\" }} and another one.";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    println!("{:?}", ast);
}

#[test]
pub fn parse_end_stmt() {
    let input = "{{!}}";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    println!("{:?}", ast);
}