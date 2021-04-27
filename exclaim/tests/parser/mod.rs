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
    let expected = r"<Ast>
</Ast>
";

    let input = "";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    
    assert_eq!(expected, &Serializer::serialize(&ast));
}

#[test]
pub fn parse_text() {
    let expected = read_file_to_string("./tests/parser/syntax/text.ast");
    let input = "Hello, World!";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_stmt_write_literals() {
    let expected = read_file_to_string("./tests/parser/syntax/stmt_write_literals.ast");
    let input = "{{ write! \"Hello!\" }}{{ write! 1234 }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_stmt_end() {
    let expected = read_file_to_string("./tests/parser/syntax/stmt_end.ast");
    let input = "This is a string. {{ ! }} and another one.";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_expr_reference() {
    let expected = read_file_to_string("./tests/parser/syntax/expr_reference.ast");
    let input = "{{ write! variable.data.field }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
#[should_panic]
fn parse_expr_reference_invalid() {
    let expected = "";
    let input = "{{ write! variable. }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_transform_literal() {
    let expected = read_file_to_string("./tests/parser/syntax/transform_literal.ast");
    let input = "{{ write! \"HELLO\" | lowercase | uppercase | lowercase }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));

}

#[test]
fn parse_transform_reference() {
    let expected = read_file_to_string("./tests/parser/syntax/transform_reference.ast");
    let input = "{{ write! site.list | enumerate }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_transform_args() {
    let expected = read_file_to_string("./tests/parser/syntax/transform_args.ast");
    let input = "{{ write! \"ABCDEFG\" | take(1,\"two\", 3) }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_stmt_let() {
    let expected = read_file_to_string("./tests/parser/syntax/stmt_let.ast");
    let input = "{{ let! x = y }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_stmt_let_pattern() {
    let expected = read_file_to_string("./tests/parser/syntax/stmt_let_pattern.ast");
    let input = "{{ let! (item, index) = list | enumerate }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
#[should_panic]
fn parse_stmt_render_empty() {
    let expected = read_file_to_string("./tests/parser/syntax/stmt_render_empty.ast");
    let input = "{{ render! }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_stmt_render_iterable() {
    let expected = read_file_to_string("./tests/parser/syntax/stmt_render_iterable.ast");
    let input = "{{ render! page : site.pages }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_sample() {
    let expected = read_file_to_string("./tests/parser/syntax/sample.ast");
    let input = read_file_to_string("./tests/parser/syntax/sample.txt");

    let tokens = exclaim::run_lexer(&input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}