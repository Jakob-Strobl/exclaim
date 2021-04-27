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