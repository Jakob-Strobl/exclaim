use std::convert;

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
}

impl convert::From<(usize, usize)> for Location {
    fn from((line, column): (usize, usize)) -> Location {
        Location {
            line,
            column
        }
    }
}