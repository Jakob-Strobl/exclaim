use serialize::Serializable;

use crate::common::serialize;
use crate::lexer::tokens::Token;

pub enum Pattern {
    Simple(SimplePattern),
    Tuple(TuplePattern)
}
impl serialize::Serializable for Pattern {
    fn serialize(&self, serde: &mut serialize::Serializer) {
        match self {
            Pattern::Simple(pattern) => pattern.serialize(serde),
            Pattern::Tuple(pattern) => pattern.serialize(serde),
        }
    }
}

pub struct SimplePattern {
    decl: Token,
}
impl SimplePattern {
    pub fn new(decl: Token) -> SimplePattern {
        SimplePattern {
            decl,
        }
    }

    pub fn decl(&self) -> &Token {
        &self.decl
    }
}
impl serialize::Serializable for SimplePattern {
    fn serialize(&self, serde: &mut serialize::Serializer) {
        let _pattern = serde.open_tag("SimplePattern");
        let _decl = serde.open_tag("decl");
        self.decl.serialize(serde)
    }
}

pub struct TuplePattern {
    decls: Vec<Token>,
}
impl TuplePattern {
    pub fn new(decls: Vec<Token>) -> TuplePattern {
        TuplePattern {
            decls,
        }
    }

    pub fn decls(&self) -> &Vec<Token> {
        &self.decls
    }
}
impl serialize::Serializable for TuplePattern {
    fn serialize(&self, serde: &mut serialize::Serializer) {
        let _pattern = serde.open_tag("TuplePattern");
        for decl in &self.decls {
            let _decl = serde.open_tag("decl");
            decl.serialize(serde)
        }
    }
}