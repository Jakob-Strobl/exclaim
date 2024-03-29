use std::collections::BTreeMap;

use crate::common::{
    PrettyString, 
    read_file_to_string
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
    let input = r#"{{ write! "ABCDEFG" | chars | get(2) }}"#;
    let expected = "Some(\"C\")";

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
<li>{{ write! chars | get(1) | unwrap }}: {{ write! chars | get(0) | unwrap }}</li>
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
    let expected = r#"The value for x is Some(144)"#;

    let mut data = DataContext::new();
    data.insert("x".to_string(), Data::Uint(144));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_object() {
    let input = r#"The object contains:
name: {{ write! object.name | unwrap }}
lang: {{ write! object | unwrap | get("lang") | unwrap }}
version: {{ write! object.version | unwrap }}
"#;
    let expected = r#"The object contains:
name: exclaim
lang: rust
version: 0.1
"#;

    let mut object = BTreeMap::new();
    object.insert("name".to_string(), Data::String("exclaim".to_string()));
    object.insert("lang".to_string(), Data::String("rust".to_string()));
    object.insert("version".to_string(), Data::Float(0.1));

    let mut data = DataContext::new();
    data.insert("object".to_string(), Data::Object(object));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_sample_product() {
    let input = read_file_to_string("./tests/runtime/input/product.html");
    let expected = read_file_to_string("./tests/runtime/output/product.html");

    let mut page = BTreeMap::new();
    page.insert("title".to_string(), Data::String("Awesome Product".to_string()));
    page.insert("header".to_string(), Data::String("Awesome Product: A product".to_string()));
    page.insert("body".to_string(), Data::String("Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
    In pharetra, nunc id posuere vestibulum, turpis nunc dapibus eros, vitae malesuada nisi tortor nec turpis. \
    Vestibulum ex leo, rhoncus bibendum urna.".to_string()));


    let mut customers = Vec::new();

    let mut customer = BTreeMap::new();
    customer.insert("name".to_string(), Data::String("John Doe".to_string()));
    customer.insert("review".to_string(), Data::String("Literally 10/10. This product is game changing.".to_string()));
    customers.push(Data::Object(customer));

    let mut customer = BTreeMap::new();
    customer.insert("name".to_string(), Data::String("Jane Doe".to_string()));
    customer.insert("review".to_string(), Data::String("My husband loves this product!".to_string()));
    customers.push(Data::Object(customer));

    let mut customer = BTreeMap::new();
    customer.insert("name".to_string(), Data::String("Anonymous".to_string()));
    customer.insert("review".to_string(), Data::String("The product name checks out.".to_string()));
    customers.push(Data::Object(customer));

    let mut customer = BTreeMap::new();
    customer.insert("name".to_string(), Data::String("Reed Salad".to_string()));
    customer.insert("review".to_string(), Data::String("It's aight".to_string()));
    customers.push(Data::Object(customer));

    let mut data = DataContext::new();
    data.insert("page".to_string(), Data::Object(page));
    data.insert("customers".to_string(), Data::Array(customers));

    let output = exclaim::run(&input, Some(data));
    pretty_assertions::assert_eq!(&output, &expected)
}

#[test]
fn render_unicode_alphabetic() {
    let input = r#"The value for Ψ is {{ write! Ψ }}"#;
    let expected = r#"The value for Ψ is Some("Psi")"#;

    let mut data = DataContext::new();
    data.insert("Ψ".to_string(), Data::String("Psi".to_string()));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_option_some() {
    let input = r#"The value may exist: {{ write! data }}"#;
    let expected = r#"The value may exist: Some("value")"#;

    let mut data = DataContext::new();
    data.insert("data".to_string(), Data::String("value".to_string()));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_option_none() {
    let input = r#"The value may exist: {{ write! data }}"#;
    let expected = r#"The value may exist: None"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_option_unwrapped() {
    let input = r#"The value may exist: {{ write! data | unwrap }}"#;
    let expected = r#"The value may exist: value"#;

    let mut data = DataContext::new();
    data.insert("data".to_string(), Data::String("value".to_string()));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

// Scalar to Scalar tests
#[test]
fn render_string_to_uint() {
    let input = r#"A string into uint: {{ write! "2021" | uint }}"#;
    let expected = r#"A string into uint: 2021"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_string_to_int() {
    let input = r#"A string into int: {{ write! "-2021" | int }}"#;
    let expected = r#"A string into int: -2021"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}  

#[test]
fn render_string_to_float() {
    let input = r#"A string into float: {{ write! "3.14" | float }}"#;
    let expected = r#"A string into float: 3.14"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_int_to_string() {
    let input = r#"A int into digits: {{ write! -1234 | string | chars }}"#;
    let expected = r#"A int into digits: ["-", "1", "2", "3", "4"]"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_int_to_uint() {
    // yeah...
    let input = r#"A int into uint: {{ write! 1234 | int | uint }}"#;
    
    let _output = exclaim::run(input, None);
}

#[test]
#[should_panic(expected="Unable to transform a negative integer into an unsigned integer")]
fn render_negative_int_to_uint() {
    let input = r#"A int into uint: {{ write! -1234 | uint }}"#;
    
    let _output = exclaim::run(input, None);
}

#[test]
fn render_int_to_float() {
    let input = r#"A int into float: {{ write! -1234 | float }}"#;
    let expected = r#"A int into float: -1234"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_uint_to_string() {
    let input = r#"A uint into digits: {{ write! 1234 | string | chars }}"#;
    let expected = r#"A uint into digits: ["1", "2", "3", "4"]"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_uint_to_int() {
    let input = r#"A uint into int: {{ write! 1234 | int }}"#;
    let expected = r#"A uint into int: 1234"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_uint_to_float() {
    let input = r#"A uint into float: {{ write! 1234 | float }}"#;
    let expected = r#"A uint into float: 1234"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_float_to_string() {
    let input = r#"A float into digits: {{ write! 3.14 | string | chars }}"#;
    let expected = r#"A float into digits: ["3", ".", "1", "4"]"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_float_to_int() {
    let input = r#"A float into int: {{ write! -3.14 | int }}"#;
    let expected = r#"A float into int: -3"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_float_to_uint() {
    let input = r#"A float into uint: {{ write! 3.14 | uint }}"#;
    let expected = r#"A float into uint: 3"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

// Compound to Compound tests
#[test]
fn render_tuple_to_tuple() {
    let input = r#"The 2D position is: {{ write! position | unwrap | tuple }}"#;
    let expected = r#"The 2D position is: (2, 5)"#;

    let mut data = DataContext::new();
    let position = Box::new([Data::Uint(2), Data::Uint(5)]);
    data.insert("position".to_string(), Data::Tuple(position));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_tuple_to_object() {
    let input = r#"The 2D position is: {{ let! object = position | unwrap | object }}{{ write! object | get("0") | unwrap }}, {{ write! object | get("1") | unwrap }}"#;
    let expected = r#"The 2D position is: 2, 5"#;

    let mut data = DataContext::new();
    let position = Box::new([Data::Uint(2), Data::Uint(5)]);
    data.insert("position".to_string(), Data::Tuple(position));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_tuple_to_array() {
    let input = r#"The 2D position is: {{ write! position | unwrap | array }}"#;
    let expected = r#"The 2D position is: [2, 5]"#;

    let mut data = DataContext::new();
    let position = Box::new([Data::Uint(2), Data::Uint(5)]);
    data.insert("position".to_string(), Data::Tuple(position));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_object_to_tuple() {
    let input = r#"Account details: {{ let! (keys, values) = account | unwrap | tuple }}{{ write! keys }} | {{ write! values }}"#;
    let expected = r#"Account details: ["dob", "location", "name"] | ["???", "Milky Way Galaxy", "Earth"]"#;

    let mut data = DataContext::new();
    let mut object = BTreeMap::new();
    object.insert("name".to_string(), Data::String("Earth".to_string()));
    object.insert("location".to_string(), Data::String("Milky Way Galaxy".to_string()));
    object.insert("dob".to_string(), Data::String("???".to_string()));
    data.insert("account".to_string(), Data::Object(object));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_object_to_object() {
    let input = r#"Account details: {{ write! account | unwrap | object }}"#;
    let expected = r#"Account details: {"dob": "???", "location": "Milky Way Galaxy", "name": "Earth"}"#;

    let mut data = DataContext::new();
    let mut object = BTreeMap::new();
    object.insert("name".to_string(), Data::String("Earth".to_string()));
    object.insert("location".to_string(), Data::String("Milky Way Galaxy".to_string()));
    object.insert("dob".to_string(), Data::String("???".to_string()));
    data.insert("account".to_string(), Data::Object(object));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_object_to_array() {
    let input = r#"Account details: {{ let! array = account | unwrap | array }}{{ write! array }}"#;
    let expected = r#"Account details: [("dob", "???"), ("location", "Milky Way Galaxy"), ("name", "Earth")]"#;

    let mut data = DataContext::new();
    let mut object = BTreeMap::new();
    object.insert("name".to_string(), Data::String("Earth".to_string()));
    object.insert("location".to_string(), Data::String("Milky Way Galaxy".to_string()));
    object.insert("dob".to_string(), Data::String("???".to_string()));
    data.insert("account".to_string(), Data::Object(object));
    
    let output = exclaim::run(input, Some(data));
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_array_to_tuple() {
    let input = r#"An array into a tuple: {{ write! "ABC" | chars | tuple }}"#;
    let expected = r#"An array into a tuple: ("A", "B", "C")"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_array_to_object() {
    let input = r#"An array into an object: {{ write! "ABC" | chars | object }}"#;
    let expected = r#"An array into an object: {"0": "A", "1": "B", "2": "C"}"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_array_to_array() {
    let input = r#"An array into an array: {{ write! "ABC" | chars | array }}"#;
    let expected = r#"An array into an array: ["A", "B", "C"]"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn render_string_length() {
    let input = r#"The length of {{ write! "ABC" }} is {{ write! "ABC" | len }}."#;
    let expected = r#"The length of ABC is 3."#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn runtime_string_concat_array() {
    let input = r#"{{ write! "ABCDEFG" | chars | concat }}"#;
    let expected = r#"ABCDEFG"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn runtime_string_concat_argument() {
    let input = r#"{{ let! name = "Earth" }}{{ write! "Hello, " | concat(name) | concat("!") }}"#;
    let expected = r#"Hello, Earth!"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}

#[test]
fn runtime_take_upper_lower() {
    let input = r#"The string {{ write! "ABCDEFG" }} sliced from 2..5: {{ write! "ABCDEFG" | chars | take(2,5) | concat }}"#;
    let expected = r#"The string ABCDEFG sliced from 2..5: CDE"#;
    
    let output = exclaim::run(input, None);
    pretty_assertions::assert_eq!(&output, expected)
}