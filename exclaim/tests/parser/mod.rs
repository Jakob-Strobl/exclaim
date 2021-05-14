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
pub fn parse_empty_input() {
    let input = "";
    let expected = r"<Ast>
</Ast>
";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    
    assert_eq!(&Serializer::serialize(&ast), expected);
}

#[test]
pub fn parse_text() {
    let input = "Hello, World!";
    let expected = read_file_to_string("./tests/parser/output/text.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_stmt_write_literals() {
    let input = "{{ write! \"Hello!\" }}{{ write! 1234 }}";
    let expected = read_file_to_string("./tests/parser/output/stmt_write_literals.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_stmt_end() {
    let input = "This is a string. {{ ! }} and another one.";
    let expected = read_file_to_string("./tests/parser/output/stmt_end.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_expr_reference() {
    let input = "{{ write! variable.data.field }}";
    let expected = read_file_to_string("./tests/parser/output/expr_reference.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
#[should_panic]
fn parse_expr_reference_invalid() {
    let input = "{{ write! variable. }}";
    let expected = "";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_transform_literal() {
    let input = "{{ write! \"HELLO\" | lowercase | uppercase | lowercase }}";
    let expected = read_file_to_string("./tests/parser/output/transform_literal.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_transform_reference() {
    let input = "{{ write! site.list | enumerate }}";
    let expected = read_file_to_string("./tests/parser/output/transform_reference.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_transform_args() {
    let input = "{{ write! \"ABCDEFG\" | take(1,\"two\", 3) }}";
    let expected = read_file_to_string("./tests/parser/output/transform_args.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_stmt_let() {
    let input = "{{ let! x = y }}";
    let expected = read_file_to_string("./tests/parser/output/stmt_let.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_stmt_let_pattern() {
    let input = "{{ let! (item, index) = list | enumerate }}";
    let expected = read_file_to_string("./tests/parser/output/stmt_let_pattern.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
#[should_panic]
fn parse_stmt_render_empty() {
    let input = "{{ render! }}";
    let expected = "";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), expected);
}

#[test]
fn parse_stmt_render_iterable() {
    let input = "{{ render! page : site.pages }}";
    let expected = read_file_to_string("./tests/parser/output/stmt_render_iterable.ast");

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}

#[test]
fn parse_sample() {
    let input = read_file_to_string("./tests/parser/input/sample.txt");
    let expected = read_file_to_string("./tests/parser/output/sample.ast");

    let tokens = exclaim::run_lexer(&input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&Serializer::serialize(&ast), &expected);
}