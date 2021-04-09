use super::ast::*;
use super::node::*;
use crate::lexer::tokens::*;
use crate::util::Location;

// Serialize the AST in an XML format 
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
            Node::Text(text) => { 
                AstSerializer::tag(
                    self, 
                    "TextNode",
                    |serde| serde.serialize_text_node(text)
                );
            }
            Node::Block(block) => {
                AstSerializer::tag(
                    self, 
                    "BlockNode",
                    |serde| serde.serialize_block_node(block)
                );
            },
            Node::Stmt(stmt) => self.serialize_stmt_node(stmt),
        }
    }

    fn serialize_text_node(&mut self, text: &TextNode) {
        AstSerializer::tag(
            self,
            "text", 
            |serde| serde.serialize_token(text.text())
        );
    }

    fn serialize_block_node(&mut self, block: &BlockNode) {
        AstSerializer::tag(
            self, 
            "open",
            |serde| serde.serialize_token(block.open())
        );

        AstSerializer::tag(
            self, 
            "stmt",
            |serde| {
                if serde.serialize_option(block.stmt()) {
                    serde.serialize_stmt_node(block.stmt().as_ref().unwrap())
                }
            }
        );

        AstSerializer::tag(
            self, 
            "close",
            |serde| {
                if serde.serialize_option(block.close()) {
                    serde.serialize_token(block.close().as_ref().unwrap())
                }
            }
        )
    }

    fn serialize_stmt_node(&mut self, stmt: &StmtNode) {

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

    fn serialize_option<T>(&mut self, option: &Option<T>) -> bool {
        match option {
            Some(_) => true,
            None => {
                self.push("None");
                false
            }
        }
    }
}



