use std::rc::Rc;
use std::cell::RefCell;

pub trait Serializable {
    fn serialize(&self, serde: &mut Serializer);
}

impl<T: Serializable> Serializable for Option<T> {
    fn serialize(&self, serde: &mut Serializer) {
        match self {
            Some(val) => {
                let _option = serde.open_tag("Option");
                val.serialize(serde);
            }
            None => {
                serde.terminal("Option", "None");
            }
        }
    }
}

impl<T: Serializable> Serializable for Box<T> {
    fn serialize(&self, serde: &mut Serializer) {
        self.as_ref().serialize(serde)
    }
}

// For placeholders of implemented types 
impl Serializable for () {
    fn serialize(&self, serde: &mut Serializer) {
        serde.push("()")
    }
}

pub struct Serializer {
    indent: Rc<RefCell<usize>>,
    buffer: Rc<RefCell<String>>,
}

impl Serializer {
    pub fn serialize(item: &dyn Serializable) -> String {
        let mut serde = Serializer::new();
        item.serialize(&mut serde);
        serde.buffer.take()
    }

    pub fn open_tag<'a>(&mut self, name: &'a str) -> Tag<'a> {
        self.indented_push(&format!("<{}>\n", name));
        self.indent();

        Tag::new(name, self.clone())
    }

    /// Takes ownership of tag and closes it 
    pub fn close_tag<'a>(&mut self, _: Tag<'a>) {
        // Takes ownership of tag and drops it 
    }

    /// Opens a self closing tag to print
    pub fn tag(serde: &mut Serializer, name: &str, nested_fn: impl Fn(&mut Serializer)) {
        serde.indented_push(&format!("<{}>\n", name));
        serde.indent();

        (nested_fn)(serde);

        serde.outdent();
        serde.indented_push(&format!("</{}>\n", name));
    }

    /// Terminal is similar to a tag, but all printed on one line; good for printing leaf nodes/fields
    pub fn terminal(&mut self, name: &str, content: &str) {
        self.indented_push(&format!("<{}>", name));
        self.indent();

        self.push(content);

        self.outdent();
        self.push(&format!("</{}>\n", name));
    }

    fn new() -> Serializer {
        Serializer { 
            indent: Rc::new(RefCell::new(0)),
            buffer: Rc::new(RefCell::new(String::from("\n"))),
        }
    }

    fn indent_as_str(&self) -> String {
        "  ".repeat(*self.indent.borrow())
    }

    fn indent(&mut self) {
        *self.indent.borrow_mut() += 1;
    }

    fn outdent(&mut self) {
        *self.indent.borrow_mut() -= 1;
    }
    
    fn push(&mut self, str: &str) {
        self.buffer.borrow_mut().push_str(str)
    }

    fn indented_push(&mut self, str: &str) {
        self.buffer.borrow_mut().push_str(&self.indent_as_str());
        self.buffer.borrow_mut().push_str(str);
    }
}

impl Clone for Serializer {
    fn clone(&self) -> Self {
        Serializer {
            indent: Rc::clone(&self.indent),
            buffer: Rc::clone(&self.buffer)
        }
    }
}

pub struct Tag<'a> {
    name: &'a str,
    serde: Serializer
}

impl<'a> Tag<'a> {
    pub fn new(name: &'a str, serde_clone: Serializer) -> Tag<'a> {
        Tag {
            name,
            serde: serde_clone,
        }
    }
}

impl<'a> Drop for Tag<'a> {
    fn drop(&mut self) {
        self.serde.outdent();
        self.serde.indented_push(&format!("</{}>\n", self.name))
    }
}