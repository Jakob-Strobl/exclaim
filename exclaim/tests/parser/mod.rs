use std::fmt;

use pretty_assertions::assert_eq;
use exclaim;
use exclaim::common::serialize::*;
use super::common::read_file_to_string;

// The following: 
//      PrettyString Newtype, 
//      fmt::Debug implementation, 
//      and macro_rule for assert_eq!,
// was wrriten by idubrov and improved by rfdonnelly on https://github.com/colin-kiegel/rust-pretty-assertions/issues/24
#[derive(PartialEq, Eq)]
#[doc(hidden)]
pub struct PrettyString<'a>(pub &'a str);

/// Write string as plain multi-line text to debug-formatter
impl<'a> fmt::Debug for PrettyString<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.0)
  }
}

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        pretty_assertions::assert_eq!(PrettyString($left), PrettyString($right));
    }
}

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
fn parse_pipe_literal() {
    let expected = read_file_to_string("./tests/parser/syntax/pipe_literal.ast");
    let input = "{{ write! \"HELLO\" | lowercase | uppercase | lowercase }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));

}

#[test]
fn parse_pipe_reference() {
    let expected = read_file_to_string("./tests/parser/syntax/pipe_reference.ast");
    let input = "{{ write! site.list | enumerate }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(&expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_pipe_args() {
    let expected = read_file_to_string("./tests/parser/syntax/pipe_args.ast");
    let input = "{{ write! \"ABCDEFG\" | take(1,\"2\",3) }}";

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