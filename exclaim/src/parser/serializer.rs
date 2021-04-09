use super::ast::*;
use super::node::*;
use crate::lexer::tokens::*;
use crate::util::Location;

// Serialize the AST in an XML-like format 
pub struct AstSerializer {
    indent: usize,
    buffer: String,
}

impl AstSerializer {
    pub fn serialize(ast: &Ast) -> String {
        let mut serde = AstSerializer::new();

        serde.push("\n");
        AstSerializer::tag(
            &mut serde, 
            "Ast",
            |serde| {
                for node in ast.blocks() {
                    serde.serialize_node(node);
                }
            }
        );
        serde.buffer
    }

    /// Opens a self closing tag to print
    fn tag(serde: &mut AstSerializer, name: &str, nested_fn: impl Fn(&mut AstSerializer)) {
        serde.indented_push(&format!("<{}>\n", name));
        serde.indent();
        (nested_fn)(serde);
        serde.outdent();
        serde.indented_push(&format!("</{}>\n", name));
    }

    fn terminal(serde: &mut AstSerializer, name: &str, nested_fn: impl Fn(&mut AstSerializer)) {
        serde.indented_push(&format!("<{}>", name));
        serde.indent();
        (nested_fn)(serde);
        serde.outdent();
        serde.push(&format!("</{}>\n", name));
    }

    fn new() -> AstSerializer {
        AstSerializer { 
            indent: 0,
            buffer: String::new(),
        }
    }

    fn indent_as_str(&self) -> String {
        "  ".repeat(self.indent)
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn outdent(&mut self) {
        self.indent -= 1;
    }
    
    fn push(&mut self, str: &str) {
        self.buffer.push_str(str);
    }

    fn indented_push(&mut self, str: &str) {
        self.buffer.push_str(&self.indent_as_str());
        self.buffer.push_str(str);
    }

    fn serialize_node(&mut self, node: &Node) {
        match node {
            Node::Text(text) => self.serialize_text_node(text),
            Node::Block(block) => self.serialize_block_node(block),
            Node::Stmt(stmt) => self.serialize_stmt_node(stmt),
        }
    }

    fn serialize_text_node(&mut self, text: &TextNode) {
        fn text_internals(serde: &mut AstSerializer, text: &TextNode) {
            AstSerializer::tag(
                serde,
                "text", 
                |serde| serde.serialize_token(text.text())
            );
        }

        AstSerializer::tag(
            self, 
            "TextNode",
            |serde| text_internals(serde, text)
        );
    }

    fn serialize_block_node(&mut self, block: &BlockNode) {
        fn block_internals(serde: &mut AstSerializer, block: &BlockNode) {
            AstSerializer::tag(
                serde, 
                "open",
                |serde| serde.serialize_token(block.open())
            );
    
            AstSerializer::tag(
                serde, 
                "stmt",
                |serde| {
                    serde.serialize_option(
                        block.stmt(),
                        AstSerializer::serialize_stmt_node
                    )
                }
            );
            
            AstSerializer::tag(
                serde, 
                "close",
                |serde| {
                    serde.serialize_option(
                        block.close(),
                        AstSerializer::serialize_token
                    );
                }
            );
        }

        AstSerializer::tag(
            self, 
            "BlockNode",
            |serde| block_internals(serde, block)
        );
    }

    fn serialize_stmt_node(&mut self, stmt: &StmtNode) {
        fn stmt_internals(serde: &mut AstSerializer, stmt: &StmtNode) {
            AstSerializer::tag(
                serde,
                "action",
                |serde| serde.serialize_token(stmt.action())
            );
    
            AstSerializer::tag(
                serde,
                "expr",
                |serde| {
                    serde.serialize_option(
                        stmt.expr(),
                        AstSerializer::serialize_expression
                    ) 
                }
            );
        }

        AstSerializer::tag(
            self,
            "StmtNode",
            |serde| stmt_internals(serde, stmt)
        );
    }

    fn serialize_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(literal) => self.serialize_literal_expression(literal)
        }
    }

    fn serialize_literal_expression(&mut self, literal: &LiteralExpression) {
        fn literal_internals(serde: &mut AstSerializer, literal: &LiteralExpression) {
            AstSerializer::tag(
                serde,
                "literal",
                |serde| serde.serialize_token(literal.literal())
            );
        }
        
        AstSerializer::tag(
            self, 
            "LiteralExpression",
            |serde| literal_internals(serde, literal)
        );
    }

    fn serialize_token(&mut self, token: &Token) {
        fn token_internals(serde: &mut AstSerializer, token: &Token) {
            AstSerializer::terminal(
                serde, 
                "kind", 
                |serde| serde.push(&format!("{:?}", token.kind()))
            );
            AstSerializer::terminal(
                serde,
                "lexeme",
                |serde| serde.push(&format!("{:?}", token.lexeme()))
            );
            AstSerializer::terminal(
                serde,
                "location",
                |serde| serde.serialize_location(token.location())
            );
        }

        AstSerializer::tag(
            self, 
            "Token",
            |serde| token_internals(serde, token)
        );
    }

    fn serialize_location(&mut self, location: &Location) {
        self.push(&format!("{{ {}, {} }}", location.line(), location.column()));
    }

    /// Serialize Rust Options
    /// some_callback is invoked when the option is Some(). The contained value is passed to the callback
    fn serialize_option<T>(&mut self, option: &Option<T>, some_callback: fn(&mut AstSerializer, val: &T)) {
        match option {
            Some(val) => {
                AstSerializer::tag(
                    self, 
                    "Option", 
                    |serde| some_callback(serde, val) 
                );
            }
            None => {
                AstSerializer::terminal(
                    self, 
                    "Option", 
                    |serde| serde.push("None")
                );
            }
        }
    }
}



