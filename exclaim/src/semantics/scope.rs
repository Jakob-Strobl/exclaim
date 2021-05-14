use std::fmt;

pub const FILE_SCOPE: usize = 0;

pub struct Scope {
    level: usize, 
    was_closed: bool,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            level: FILE_SCOPE,
            was_closed: false,
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn open(&mut self) {
        println!("Opened scope!");
        self.level += 1;
    }

    pub fn close(&mut self) {
        println!("Closed scope!");
        self.level -= 1;
        self.was_closed = true;
    }

    pub fn was_closed(&mut self) -> bool {
        if self.was_closed {
            self.was_closed = false;
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.level == FILE_SCOPE {
            write!(f, "File Scope")
        } else {
            write!(f, "Local Scope ({})", self.level)
        }
    }
}