#[test]
#[should_panic]
fn missing_closing_block() {
    let input = r#"
{{ render! a : b }}
    {{ write! a }}
"#;

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);
    let ast = exclaim::run_semantics(ast);
}

#[test]
#[should_panic(expected = "Invalid lone closing block in file scope.")]
fn invalid_end_block() {
    let input = r#"{{!}}"#;

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);
    let ast = exclaim::run_semantics(ast);
}