use std::convert;

use crate::ast::AstIndex;
use crate::common::serialize::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    line: usize,
    column: usize, 
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location {
            line,
            column
        }
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    } 

    pub fn shift(&mut self) {
        self.column += 1;
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl convert::From<(usize, usize)> for Location {
    fn from((line, column): (usize, usize)) -> Location {
        Location {
            line,
            column
        }
    }
}

impl  Serializable for Location {
    fn serialize(&self, serde: &mut Serializer, _: &dyn IndexSerializable) -> Option<AstIndex> {
        serde.terminal("location", format!("{{ {}, {} }}", self.line, self.column).as_str());
        None
    }
}