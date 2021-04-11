pub trait Serializable {
    fn serialize(&self, serde: &mut Serializer);
}

impl<T: Serializable> Serializable for Option<T> {
    fn serialize(&self, serde: &mut Serializer) {
        match self {
            Some(val) => {
                Serializer::tag(
                    serde, 
                    "Option", 
                    |serde| val.serialize(serde)
                );
            }
            None => {
                Serializer::terminal(
                    serde, 
                    "Option", 
                    || String::from("None")
                );
            }
        }
    }
}

impl<T: Serializable> Serializable for Box<T> {
    fn serialize(&self, serde: &mut Serializer) {
        self.as_ref().serialize(serde)
    }
}

pub struct Serializer {
    indent: usize,
    buffer: String,
}

impl Serializer {
    pub fn serialize(item: &dyn Serializable) -> String {
        let mut serde = Serializer::new();
        item.serialize(&mut serde);
        serde.buffer
    }

    /// Opens a self closing tag to print
    pub fn tag(serde: &mut Serializer, name: &str, nested_fn: impl Fn(&mut Serializer)) {
        serde.indented_push(&format!("<{}>\n", name));
        serde.indent();

        (nested_fn)(serde);

        serde.outdent();
        serde.indented_push(&format!("</{}>\n", name));
    }

    pub fn terminal(serde: &mut Serializer, name: &str, nested_fn: impl Fn() -> String) {
        serde.indented_push(&format!("<{}>", name));
        serde.indent();

        let str = nested_fn();
        serde.push(&str);

        serde.outdent();
        serde.push(&format!("</{}>\n", name));
    }

    fn new() -> Serializer {
        Serializer { 
            indent: 0,
            buffer: String::from("\n"),
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



