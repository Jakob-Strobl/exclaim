use crate::common::{
    PrettyString
};

// Overrides std lib assert_eq with PrettyString version of assert_eq. 
// You need to include common::PrettyString newtype
// Defined in 'tests/common/mod.rs'
use crate::assert_eq;

#[test]
fn render_plain_text() {
    let input = r#"This is just plain text."#;
    let expected = "This is just plain text.";

    let output = exclaim::run(input);

    assert_eq!(&output, expected);
}

#[test]
fn render_string_literal() {
    let input = r#"{{ write! "Hello" }} world!"#;
    let expected = "Hello world!";

    let output = exclaim::run(input);

    assert_eq!(&output, expected);
}