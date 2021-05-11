use std::collections::HashMap;

use crate::common::{
    PrettyString
};

use exclaim::{
    DataContext,
    Data,
};

// Overrides std lib assert_eq with PrettyString version of assert_eq. 
// You need to include common::PrettyString newtype
// Defined in 'tests/common/mod.rs'
use crate::assert_eq;

#[test]
fn render_plain_text() {
    let input = r#"This is just plain text."#;
    let expected = "This is just plain text.";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected);
}

#[test]
fn render_string_literal() {
    let input = r#"{{ write! "Hello" }} world!"#;
    let expected = "Hello world!";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected);
}

#[test]
fn render_number_literal() {
    let input = r#"Number is {{ write! 1337 }}"#;
    let expected = "Number is 1337";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected);
}

#[test]
fn render_assigned_variable_str_literal() {
    let input = r#"{{ let! notes = "ABCDEFG" }}{{ write! notes }}"#;
    let expected = "ABCDEFG";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected);
}

#[test]
fn render_assigned_variable_number_literal() {
    let input = r#"{{ let! num = 6 }}{{ write! num }}"#;
    let expected = "6";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected);
}

#[test]
fn render_transformed_literal() {
    let input = r#"{{ write! "Abc" | lowercase | uppercase }}"#;
    let expected = "ABC";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected);
}

#[test]
fn render_transformed_reference() {
    let input = r#"{{ let! string = "Hello!" }}{{ write! string | uppercase }}"#;
    let expected = "HELLO!";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected);
}

#[test]
fn render_transformed_assignment() {
    let input = r#"{{ let! string = "test 123" | uppercase }}{{ write! string }}"#;
    let expected = "TEST 123";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected)
}

#[test]
fn render_transform_argument() {
    let input = r#"{{ write! "ABCDEFG" | at(2) }}"#;
    let expected = "C";

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected)
}

#[test]
fn render_arrays() {
    let input = r#"{{ write! "ABCDEFG" | chars }}"#;
    let expected = r#"["A", "B", "C", "D", "E", "F", "G"]"#;

    let output = exclaim::run(input, None);
    assert_eq!(&output, expected)
}

#[test]
fn render_render_statement() {
    let input = r#"{{ render! chs : "ABC" | chars }}
{{ write! chs }}
{{!}}"#;
    let expected = r#"
A

B

C
"#;

    let output = exclaim::run(input, None);
    // So uhhh, for somereason using the custom pretty assert causes this to crash.
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_enumerate() {
    let input = r#"{{ render! (ch, index) : "ABC" | chars | enumerate }}
<li>{{ write! index }}: {{ write! ch }}</li>
{{!}}"#;
        let expected = r#"
<li>0: A</li>

<li>1: B</li>

<li>2: C</li>
"#;
    
    let output = exclaim::run(input, None);
    // So uhhh, for somereason using the custom pretty assert causes this to crash.
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_tuple_indexing() {
    let input = r#"{{ render! chars : "ABC" | chars | enumerate }}
<li>{{ write! chars | at(1) }}: {{ write! chars | at(0) }}</li>
{{!}}"#;
    let expected = r#"
<li>0: A</li>

<li>1: B</li>

<li>2: C</li>
"#;
    
    let output = exclaim::run(input, None);
    // So uhhh, for somereason using the custom pretty assert causes this to crash.
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_global_number() {
    let input = r#"The value for x is {{ write! x }}"#;
    let expected = r#"The value for x is 144"#;

    let mut data = DataContext::new();
    data.insert("x".to_string(), Data::Uint(144));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_object() {
    let input = r#"The object contains:
name: {{ write! object.name }}
lang: {{ write! object | get("lang") }}
version: {{ write! object.version }}
"#;
    let expected = r#"The object contains:
name: exclaim
lang: rust
version: 0.1
"#;

    let mut object = HashMap::new();
    object.insert("name".to_string(), Data::String("exclaim".to_string()));
    object.insert("lang".to_string(), Data::String("rust".to_string()));
    object.insert("version".to_string(), Data::Float(0.1));

    let mut data = DataContext::new();
    data.insert("object".to_string(), Data::Object(object));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}