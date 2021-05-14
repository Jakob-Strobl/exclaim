use crate::tokens::Token;
use crate::common::Location;

pub struct StackMachine {
    chars: Vec<char>,
    index: usize,
    stack: String,
    tokens: Vec<Token>,
    // Keeps track of start location of a token 
    start: Location,
    // Keeps track of current location in the input 
    current: Location,
}

impl StackMachine {
    pub fn new<S: AsRef<str>>(input: S) -> StackMachine {
        StackMachine {
            chars: input.as_ref().chars().collect(),
            index: 0,
            stack: String::new(),
            tokens: Vec::new(),
            start: Location::new(0, 0),
            current: Location::new(0, 0)
        }
    }

    pub fn view_stack(&self) -> &str {
        self.stack.as_str()
    }

    pub fn peek(&self) -> char {
        self.chars[self.index]
    }

    pub fn lookahead(&self) -> Option<&char> {
        self.chars.get(self.index + 1)
    }

    pub fn skip(&mut self) {
        self.index += 1;

        // Shift both locations up
        self.start.shift();
        self.current.shift();
    }

    /// Skips the current character and only shifts the current location. Start location does not change.
    pub fn skip_current(&mut self) {
        self.index += 1;

        // Shift both locations up
        self.current.shift();
    }

    pub fn push(&mut self) {
        let ch = self.chars[self.index];
        self.stack.push(ch);
        self.index += 1;

        // Shift just current location
        self.current.shift();
    }

    pub fn newline(&mut self) {
        self.current.newline();
    }

    pub fn location(&self) -> Location {
        self.start.clone()
    }

    pub fn accept_token(&mut self, token: Token) {
        self.tokens.push(token);

        // Set for new fresh token
        self.stack = String::new();
        self.start = self.current;
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub fn empty(&self) -> bool {
        self.stack.len() == 0
    }

    pub fn eof(&self) -> bool {
        self.index >= self.chars.len()
    }

    pub fn debug_line(&self, underline_msg: &str) -> (Location, String) {
        let mut index = self.index; 
        let mut line = String::new();

        // Backup to get starting character of the line 
        for idx in (0..=index).rev() {
            if self.chars[idx] == '\n' {
                break;
            }
            index = idx;
        }

        // Collect every character until we reach newline or eof 
        while let Some(ch) = self.chars.get(index) {
            if *ch == '\n' {
                break;
            }
            line.push(self.chars[index]);
            index += 1;
        }

        // Underline location of error 
        line.push('\n');
        line.push('\t');
        line.push_str(&" ".repeat(self.current.column()));
        line.push_str(&"^");
        line.push(' ');
        line.push_str(&underline_msg);

        (self.current, line)
    }
}