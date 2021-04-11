use super::ast::*;

pub trait Serializeable {
    fn serialize(&self, serde: &mut AstSerializer);
}

impl<T: Serializeable> Serializeable for Option<T> {
    fn serialize(&self, serde: &mut AstSerializer) {
        match self {
            Some(val) => {
                AstSerializer::tag(
                    serde, 
                    "Option", 
                    |serde| val.serialize(serde)
                );
            }
            None => {
                AstSerializer::terminal(
                    serde, 
                    "Option", 
                    || String::from("None")
                );
            }
        }
    }
}

impl<T: Serializeable> Serializeable for Box<T> {
    fn serialize(&self, serde: &mut AstSerializer) {
        self.as_ref().serialize(serde)
    }
}


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
                    node.serialize(serde)
                }
            }
        );
        serde.buffer
    }

    /// Opens a self closing tag to print
    pub fn tag(serde: &mut AstSerializer, name: &str, nested_fn: impl Fn(&mut AstSerializer)) {
        serde.indented_push(&format!("<{}>\n", name));
        serde.indent();
        (nested_fn)(serde);
        serde.outdent();
        serde.indented_push(&format!("</{}>\n", name));
    }

    pub fn terminal(serde: &mut AstSerializer, name: &str, nested_fn: impl Fn() -> String) {
        serde.indented_push(&format!("<{}>", name));
        serde.indent();

        let str = nested_fn();
        serde.push(&str);

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
}



