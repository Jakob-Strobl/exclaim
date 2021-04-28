use exclaim::common::serialize::*;
use crate::common::{
    PrettyString,
    read_file_to_string,
};

// Overrides std lib assert_eq with PrettyString version of assert_eq. 
// You need to include common::PrettyString newtype
// Defined in 'tests/common/mod.rs'
use crate::assert_eq;

#[test]
#[should_panic(expected = "Expected the scope to be closed with {{!}}")]
fn missing_closing_block() {
    let input = r#"
{{ render! a : b }}
    {{ write! a }}
"#;

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);
    let _ast = exclaim::run_semantics(ast);
}

#[test]
#[should_panic(expected = "Invalid lone closing block in file scope.")]
fn invalid_end_block() {
    let input = r#"{{!}}"#;

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);
    let _ast = exclaim::run_semantics(ast);
}

#[test]
fn unclosed_block_scope_shallow() {
    let input = r#"
{{ render! a : b }}
<p>{{ write! a }}</p>
{{!}}
Text
"#;
    let expected = read_file_to_string("./tests/semantics/output/unclosed_block_scope_shallow.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);
    let ast = exclaim::run_semantics(ast);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn unclosed_block_scope_deep() {
    let input = r#"
Start File Scope
{{ render! a : b }}
Start Scope 1
{{ render! c : d }}
Start Scope 2
{{ render! e : f }}
Scope 3
{{!}}
Scope 2
{{ render! e : f }}
Scope 3 part 2
{{!}}
End Scope 2
{{!}}
End Scope 1
{{!}}
End File Scope
"#;

    let expected = read_file_to_string("./tests/semantics/output/unclosed_block_scope_deep.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);
    let ast = exclaim::run_semantics(ast);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}