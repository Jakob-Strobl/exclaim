use std::fmt;
use pretty_assertions::assert_eq;
use exclaim;
use exclaim::common::serialize::*;

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

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(expected, &Serializer::serialize(&ast));
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

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(expected, &Serializer::serialize(&ast));
}

#[test]
pub fn parse_write_string() {
    let expected = r#"
<Ast>
  <BlockNode>
    <stmt>
      <SimpleStatement>
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
              <pipe>
                <Option>None</Option>
              </pipe>
            </LiteralExpression>
          </Option>
        </expr>
      </SimpleStatement>
    </stmt>
  </BlockNode>
</Ast>
"#;
    let input = "{{ write! \"Hello!\" }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(expected, &Serializer::serialize(&ast));
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
      <SimpleStatement>
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
      </SimpleStatement>
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

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(expected, &Serializer::serialize(&ast));
}

#[test]
pub fn parse_references() {
  let expected = r#"
<Ast>
  <BlockNode>
    <stmt>
      <SimpleStatement>
        <action>
          <Token>
            <kind>Action(Write)</kind>
            <lexeme>"write!"</lexeme>
            <location>{ 0, 3 }</location>
          </Token>
        </action>
        <expr>
          <Option>
            <ReferenceExpression>
              <reference>
                <Token>
                  <kind>Label</kind>
                  <lexeme>"variable"</lexeme>
                  <location>{ 0, 10 }</location>
                </Token>
              </reference>
              <child>
                <Option>
                  <ReferenceExpression>
                    <reference>
                      <Token>
                        <kind>Label</kind>
                        <lexeme>"data"</lexeme>
                        <location>{ 0, 19 }</location>
                      </Token>
                    </reference>
                    <child>
                      <Option>
                        <ReferenceExpression>
                          <reference>
                            <Token>
                              <kind>Label</kind>
                              <lexeme>"field"</lexeme>
                              <location>{ 0, 24 }</location>
                            </Token>
                          </reference>
                          <child>
                            <Option>None</Option>
                          </child>
                        </ReferenceExpression>
                      </Option>
                    </child>
                  </ReferenceExpression>
                </Option>
              </child>
            </ReferenceExpression>
          </Option>
        </expr>
      </SimpleStatement>
    </stmt>
  </BlockNode>
</Ast>
"#;

    let input = "{{ write! variable.data.field }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_pipes_on_literal() {
    let expected = r#"
<Ast>
  <BlockNode>
    <stmt>
      <SimpleStatement>
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
                  <lexeme>"HELLO"</lexeme>
                  <location>{ 0, 10 }</location>
                </Token>
              </literal>
              <pipe>
                <Option>
                  <PipeSubExpression>
                    <call>
                      <Call>
                        <function>
                          <Token>
                            <kind>Label</kind>
                            <lexeme>"lowercase"</lexeme>
                            <location>{ 0, 20 }</location>
                          </Token>
                        </function>
                        <arguments>
                          <Option>None</Option>
                        </arguments>
                      </Call>
                    </call>
                    <next>
                      <Option>
                        <PipeSubExpression>
                          <call>
                            <Call>
                              <function>
                                <Token>
                                  <kind>Label</kind>
                                  <lexeme>"uppercase"</lexeme>
                                  <location>{ 0, 32 }</location>
                                </Token>
                              </function>
                              <arguments>
                                <Option>None</Option>
                              </arguments>
                            </Call>
                          </call>
                          <next>
                            <Option>
                              <PipeSubExpression>
                                <call>
                                  <Call>
                                    <function>
                                      <Token>
                                        <kind>Label</kind>
                                        <lexeme>"lowercase"</lexeme>
                                        <location>{ 0, 44 }</location>
                                      </Token>
                                    </function>
                                    <arguments>
                                      <Option>None</Option>
                                    </arguments>
                                  </Call>
                                </call>
                                <next>
                                  <Option>None</Option>
                                </next>
                              </PipeSubExpression>
                            </Option>
                          </next>
                        </PipeSubExpression>
                      </Option>
                    </next>
                  </PipeSubExpression>
                </Option>
              </pipe>
            </LiteralExpression>
          </Option>
        </expr>
      </SimpleStatement>
    </stmt>
  </BlockNode>
</Ast>
"#;
    let input = "{{ write! \"HELLO\" | lowercase | uppercase | lowercase }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(expected, &Serializer::serialize(&ast));

}

#[test]
fn parse_call_with_args() {
    let expected = r#"
<Ast>
  <BlockNode>
    <stmt>
      <SimpleStatement>
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
                  <lexeme>"ABCDEFG"</lexeme>
                  <location>{ 0, 10 }</location>
                </Token>
              </literal>
              <pipe>
                <Option>
                  <PipeSubExpression>
                    <call>
                      <Call>
                        <function>
                          <Token>
                            <kind>Label</kind>
                            <lexeme>"take"</lexeme>
                            <location>{ 0, 22 }</location>
                          </Token>
                        </function>
                        <arguments>
                          <Option>
                            <Arguments>
                              <arg>
                                <LiteralExpression>
                                  <literal>
                                    <Token>
                                      <kind>NumberLiteral(1)</kind>
                                      <lexeme>"1"</lexeme>
                                      <location>{ 0, 27 }</location>
                                    </Token>
                                  </literal>
                                  <pipe>
                                    <Option>None</Option>
                                  </pipe>
                                </LiteralExpression>
                              </arg>
                              <arg>
                                <LiteralExpression>
                                  <literal>
                                    <Token>
                                      <kind>StringLiteral</kind>
                                      <lexeme>"2"</lexeme>
                                      <location>{ 0, 29 }</location>
                                    </Token>
                                  </literal>
                                  <pipe>
                                    <Option>None</Option>
                                  </pipe>
                                </LiteralExpression>
                              </arg>
                              <arg>
                                <LiteralExpression>
                                  <literal>
                                    <Token>
                                      <kind>NumberLiteral(3)</kind>
                                      <lexeme>"3"</lexeme>
                                      <location>{ 0, 33 }</location>
                                    </Token>
                                  </literal>
                                  <pipe>
                                    <Option>None</Option>
                                  </pipe>
                                </LiteralExpression>
                              </arg>
                            </Arguments>
                          </Option>
                        </arguments>
                      </Call>
                    </call>
                    <next>
                      <Option>None</Option>
                    </next>
                  </PipeSubExpression>
                </Option>
              </pipe>
            </LiteralExpression>
          </Option>
        </expr>
      </SimpleStatement>
    </stmt>
  </BlockNode>
</Ast>
"#;
    let input = "{{ write! \"ABCDEFG\" | take(1,\"2\",3) }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_let_stmt() {
    let expected = r#"
<Ast>
  <BlockNode>
    <stmt>
      <LetStatement>
        <assignee>
          <SimplePattern>
            <decl>
              <Token>
                <kind>Label</kind>
                <lexeme>"x"</lexeme>
                <location>{ 0, 8 }</location>
              </Token>
            </decl>
          </SimplePattern>
        </assignee>
        <expr>
          <ReferenceExpression>
            <reference>
              <Token>
                <kind>Label</kind>
                <lexeme>"y"</lexeme>
                <location>{ 0, 12 }</location>
              </Token>
            </reference>
            <child>
              <Option>None</Option>
            </child>
          </ReferenceExpression>
        </expr>
      </LetStatement>
    </stmt>
  </BlockNode>
</Ast>
"#;
    let input = "{{ let! x = y }}";

    let tokens = exclaim::run_lexer(input);
    let ast = exclaim::run_parser(tokens);

    assert_eq!(expected, &Serializer::serialize(&ast));
}

#[test]
fn parse_let_stmt_pattern() {
  let expected = r#""#;
  let input = "{{ let! (item, index) = list | enumerate }}";

  let tokens = exclaim::run_lexer(input);
  let ast = exclaim::run_parser(tokens);

  assert_eq!(expected, &Serializer::serialize(&ast));
}