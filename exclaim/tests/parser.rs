use std::fmt;
use pretty_assertions::assert_eq;
use exclaim::*;

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
    let expected = r"
<Ast>
</Ast>
";

    let input = "";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, &AstSerializer::serialize(&ast));
}

#[test]
pub fn parse_string_literal() {
    let expected = r#"
<Ast>
  <TextNode>
    <text>
      <Token>
        <kind>StringLiteral</kind>
        <lexeme>"Hello, World!"</lexeme>
        <location>{ 0, 0 }</location>
      </Token>
    </text>
  </TextNode>
</Ast>
"#;
    let input = "Hello, World!";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, &AstSerializer::serialize(&ast));
}

#[test]
pub fn parse_write_string() {
    let expected = r#"
<Ast>
  <BlockNode>
    <stmt>
      <Option>
        <StmtNode>
          <action>
            <Token>
              <kind>Action(Write)</kind>
              <lexeme>"write!"</lexeme>
              <location>{ 0, 3 }</location>
            </Token>
          </action>
          <expr>
            <Option>
              <LiteralExpression>
                <literal>
                  <Token>
                    <kind>StringLiteral</kind>
                    <lexeme>"Hello!"</lexeme>
                    <location>{ 0, 10 }</location>
                  </Token>
                </literal>
              </LiteralExpression>
            </Option>
          </expr>
        </StmtNode>
      </Option>
    </stmt>
  </BlockNode>
</Ast>
"#;
    let input = "{{ write! \"Hello!\" }}";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, &AstSerializer::serialize(&ast));
}

#[test]
pub fn parse_end_stmt() {
    let expected = r#"
<Ast>
  <TextNode>
    <text>
      <Token>
        <kind>StringLiteral</kind>
        <lexeme>"This is a string. "</lexeme>
        <location>{ 0, 0 }</location>
      </Token>
    </text>
  </TextNode>
  <BlockNode>
    <stmt>
      <Option>
        <StmtNode>
          <action>
            <Token>
              <kind>Action(End)</kind>
              <lexeme>"!"</lexeme>
              <location>{ 0, 21 }</location>
            </Token>
          </action>
          <expr>
            <Option>None</Option>
          </expr>
        </StmtNode>
      </Option>
    </stmt>
  </BlockNode>
  <TextNode>
    <text>
      <Token>
        <kind>StringLiteral</kind>
        <lexeme>" and another one."</lexeme>
        <location>{ 0, 25 }</location>
      </Token>
    </text>
  </TextNode>
</Ast>
"#;
    let input = "This is a string. {{ ! }} and another one.";

    let tokens = crate::run_lexer(input);
    let ast = crate::run_parser(tokens);

    assert_eq!(expected, &AstSerializer::serialize(&ast));
}