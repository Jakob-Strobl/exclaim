use std::convert;
use std::fs::File;
use std::io::Read;

use crate::tokens::{Token, TokenKind}; 
use automata::StackMachine;
use states::State;

pub struct Lexer {
    stack: StackMachine,
    state: &'static State,
}

impl Lexer {
    /// tokenize() converts the given input into an array of tokens
    /// This method also consumes the lexer
    pub fn tokenize(mut self) -> Vec<Token> {
        while !self.stack.eof() {
            if self.stack.peek() == '\n' {
                self.state = self.state.run(&mut self.stack);
                self.stack.newline();
            } else {
                self.state = self.state.run(&mut self.stack);
            }
        }

        // consume leftovers
        if !self.stack.empty() {
            self.stack.accept_token(TokenKind::StringLiteral);
        }

        self.stack.get_tokens()
    }

    /// get_token() returns one token at a time
    /// This method does not consume the lexer
    pub fn get_token(&mut self) -> Token {
        // Keep going until we get a token! 
        while !self.stack.eof() {    
            self.state = self.state.run(&mut self.stack);
            if let Some(t) = self.stack.get_token() {
                return t
            }
        }

        // Consume leftovers
        // This might return an empty string literal 
        self.stack.accept_token(TokenKind::StringLiteral);
        self.stack.get_token().unwrap()
    }
}


impl convert::From<&str> for Lexer {
    fn from(string: &str) -> Lexer {
        Lexer {
            stack: StackMachine::new(string),
            state: State::new(),
        }
    }
}

impl convert::From<String> for Lexer {
    fn from(string: String) -> Lexer {
        Lexer {
            stack: StackMachine::new(&string),
            state: State::new(),
        }
    }
}

impl convert::From<File> for Lexer {
    fn from(mut file: File) -> Lexer {
        let mut input = String::new();

        let result = file.read_to_string(&mut input);

        match result {
            Ok(_) => {
                Lexer {
                    stack: StackMachine::new(&input),
                    state: State::new(),
                }
            },
            Err(error) => {
                panic!("<lexer::from<File>()> failed to read contents of file into a string:\n{}", error);
            }
        }
    }
}

mod states {
    use super::automata::StackMachine;
    use crate::tokens::{TokenKind, Op, Action};
    
    pub struct State(fn(&mut StackMachine) -> &'static State);

    impl State {
        pub fn new() -> &'static State {
            &STATE_START
        }

        pub fn run(&self, stack: &mut StackMachine) -> &'static State {
            &self.0(stack)
        }

        pub fn get_error_msg(stack: &mut StackMachine, msg: &str, underline_msg: &str) -> String {
            let (loc, line) = stack.debug_line(underline_msg);
            format!("{} On line [{}; {}]:\n\t{}", msg, loc.line(), loc.column(), line)
        }
    }

    static STATE_START: State = State(
        |stack| {
            match stack.peek() {
                '{' => &STATE_OPEN_BLOCK,
                '}' => &STATE_CLOSE_BLOCK,
                _ => {
                    stack.push();
                    &STATE_START
                }
            }
        }
    );

    static STATE_OPEN_BLOCK: State = State(
        |stack| {
            // Context, we already know stack.peek() == '{'
            match stack.lookahead().unwrap_or(&' ') {
                '{' => {
                    // Accept string literal if the stack is not empty, because next token is a BlockOpen 
                    if !stack.empty() {
                        stack.accept_token(TokenKind::StringLiteral);
                    }
                    &ACCEPT_OPEN_BLOCK
                },
                _ => {
                    stack.push();
                    &STATE_START
                }
            }
        }
    );

    static STATE_CLOSE_BLOCK: State = State(
        |stack| {
            // Context, we already know stack.peek() == '}'
            match stack.lookahead().unwrap_or(&' ') {
                '}' => {
                    // Accept string literal if the stack is not empty, because next token is a BlockClose 
                    if !stack.empty() {
                        stack.accept_token(TokenKind::StringLiteral);
                    }
                    &ACCEPT_CLOSE_BLOCK
                },
                _ => {
                    stack.push();
                    &STATE_START
                }
            }
        }
    );

    static ACCEPT_OPEN_BLOCK: State = State(
        |stack| {
            stack.push(); // {
            stack.push(); // {{
            stack.accept_token(TokenKind::Operator(Op::BlockOpen));
            &STATE_BLOCK
        }
    );

    static ACCEPT_CLOSE_BLOCK: State = State(
        |stack| {
            stack.push(); // }
            stack.push(); // }}
            stack.accept_token(TokenKind::Operator(Op::BlockClose));
            &STATE_START
        }
    );

    static STATE_BLOCK: State = State(
        |stack| {
            let ch = stack.peek();
            match ch {
                '{' => &STATE_OPEN_BLOCK_FROM_BLOCK,
                '}' => &STATE_CLOSE_BLOCK_FROM_BLOCK,
                '!' => &STATE_BLOCK_ACTION_INEQUALITY,
                '=' => &STATE_BLOCK_ASSIGN_EQUALITY,
                '|' => &STATE_BLOCK_PIPE_OR,
                '&' => &STATE_BLOCK_AND,
                ',' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::Comma));
                    &STATE_BLOCK
                }
                '.' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::Dot));
                    &STATE_BLOCK
                },
                ':' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::Each));
                    &STATE_BLOCK
                },
                '"' => {
                    stack.skip_current();
                    &STATE_BLOCK_STRING_LITERAL
                },
                '[' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::ClosureOpen));
                    &STATE_BLOCK
                },
                ']' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::ClosureClose));
                    &STATE_BLOCK
                },
                _ => {
                    if ch.is_alphabetic() {
                        stack.push();
                        &STATE_LABEL
                    } else if ch.is_numeric() {
                        stack.push();
                        &STATE_DIGIT
                    } else if ch.is_whitespace() {
                        stack.skip();
                        &STATE_BLOCK
                    } else {
                        panic!(State::get_error_msg(
                            stack, 
                            &format!("Lexer<BLOCK>: Encountered unknown character '{}'.", ch),
                            "unknown character",
                        ));
                    }
                }
            }
        }
    );

    static STATE_OPEN_BLOCK_FROM_BLOCK: State = State(
        |stack| {
            // Context, we already know stack.peek() == '{'
            match stack.lookahead().unwrap_or(&' ') {
                '{' => {
                    // Accept string literal if the stack is not empty, because next token is a BlockOpen 
                    if !stack.empty() {
                        stack.accept_token(TokenKind::StringLiteral);
                    }
                    &ACCEPT_OPEN_BLOCK
                },
                _ => {
                    stack.push();
                    &STATE_BLOCK
                }
            }
        }
    );

    static STATE_CLOSE_BLOCK_FROM_BLOCK: State = State(
        |stack| {
            // Context, we already know stack.peek() == '}'
            match stack.lookahead().unwrap_or(&' ') {
                '}' => {
                    // Accept string literal if the stack is not empty, because next token is a BlockClose 
                    if !stack.empty() {
                        stack.accept_token(TokenKind::StringLiteral);
                    }
                    &ACCEPT_CLOSE_BLOCK
                },
                _ => {
                    stack.push();
                    &STATE_BLOCK
                }
            }
        }
    );

    static STATE_BLOCK_ACTION_INEQUALITY: State = State(
        |stack| {
            // Context, we already know stack.peek() == '!'
            match stack.lookahead().unwrap_or(&' ') {
                '=' => {
                    stack.push(); // !
                    stack.push(); // !=
                    stack.accept_token(TokenKind::Operator(Op::Inequality));
                }
                _ => {
                    stack.push();
                    stack.accept_token(TokenKind::Action(Action::End));
                }
            }

            &STATE_BLOCK
        }
    );


    static STATE_BLOCK_ASSIGN_EQUALITY: State = State(
        |stack| {
            // Context, we already know stack.peek() == '='
            match stack.lookahead().unwrap_or(&' ') {
                '=' => {
                    stack.push(); // =
                    stack.push(); // ==
                    stack.accept_token(TokenKind::Operator(Op::Equality));
                }
                _ => {
                    stack.push(); // =
                    stack.accept_token(TokenKind::Operator(Op::Assign));
                }
            }

            &STATE_BLOCK
        }
    );

    static STATE_BLOCK_STRING_LITERAL: State = State(
        |stack| {
            match stack.peek() {
                '"' => { 
                    stack.skip_current();   // Skip closing double quote
                    stack.accept_token(TokenKind::StringLiteral);
                    &STATE_BLOCK
                },
                '\\' => { // ESCAPE CHARACTER
                    stack.skip_current();   // Skip escape
                    stack.push();           // Push character escaped
                    &STATE_BLOCK_STRING_LITERAL
                },
                _ => {
                    stack.push();
                    &STATE_BLOCK_STRING_LITERAL
                }
            }
        }
    );

    static STATE_BLOCK_AND: State = State(
        |stack| {
            // Context, we already know stack.peek() == '&'
            match stack.lookahead().unwrap_or(&' ') {
                '&' => {
                    stack.push(); // &
                    stack.push(); // &&
                    stack.accept_token(TokenKind::Operator(Op::And));
                    &STATE_BLOCK
                }
                _ => {
                    panic!(State::get_error_msg(
                        stack, 
                        "Lexer<AND>: Expected Operator And(&&). A single '&' is not a valid token.",
                        "expected '&&'",
                    ));
                }
            }
        }
    );

    static STATE_BLOCK_PIPE_OR: State = State(
        |stack| {
            // Context, we already know stack.peek() == '|'
            match stack.lookahead().unwrap_or(&' ') {
                '|' => {
                    stack.push(); // |
                    stack.push(); // || Or
                    stack.accept_token(TokenKind::Operator(Op::Or));
                }
                _ => {
                    stack.push(); // | Pipe
                    stack.accept_token(TokenKind::Operator(Op::Pipe));
                }
            }
            &STATE_BLOCK
        }
    );

    static STATE_LABEL: State = State(
        |stack| {
            let ch = stack.peek();
            if ch.is_alphabetic() {
                stack.push();
                &STATE_LABEL
            } else if ch == '!' && *stack.lookahead().unwrap_or(&' ') != '=' {
                // If the following two characters are not: !=
                // Push ! 
                stack.push();

                // Derive action variant
                let action = match stack.view_stack() {
                    // There is no way it should be an empty string, since one character has to be consumed to even be in this state.
                    "let!" => Action::Let,
                    "write!" => Action::Write,
                    "render!" => Action::Render,
                    _ => panic!(State::get_error_msg(
                                    stack, 
                                    &format!("Lexer<LABEL>: The expected action does not match a defined action - found: '{}' ", stack.view_stack()), 
                                    "expected one of the following defined actions: let!, write!, render!, or !."))
                };

                stack.accept_token(TokenKind::Action(action));
                &STATE_BLOCK

            } else if ch.is_numeric() {
                panic!(State::get_error_msg(
                    stack, 
                    &format!("Lexer<LABEL>: The expected label contains digit '{}' with stack \"{}\".", ch, stack.view_stack()),
                    "expected alphabetic character",
                ));
            } else {
                // Accept Label 
                stack.accept_token(TokenKind::Label);
                &STATE_BLOCK
            }
        }
    );

    static STATE_DIGIT: State = State(
        |stack| {
            let ch = stack.peek();
            if ch.is_numeric() {
                stack.push();
                &STATE_DIGIT
            } else if ch.is_alphabetic() {
                panic!(State::get_error_msg(
                    stack, 
                    &format!("Lexer<DIGIT>: The expected number contains invalid digit '{}' with stack \"{}\".", ch, stack.view_stack()),
                    "expected digit",
                ));
            } else {
                // Accept Number 
                let number: usize = stack.view_stack().parse::<usize>().unwrap();
                stack.accept_token(TokenKind::NumberLiteral(number));
                &STATE_BLOCK
            }
        }
    );
}

mod automata {
    use crate::tokens::{Token, TokenKind};
    use crate::util::Location;

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
        pub fn new(input: &str) -> StackMachine {
            StackMachine {
                chars: input.chars().collect(),
                index: 0,
                stack: String::new(),
                tokens: Vec::new(),
                start: Location::new(0, 0),
                current: Location::new(0, 0)
            }
        }

        /// Exposes the entire underlying input as a String
        pub fn as_string(&self) -> String {
            self.chars.iter().collect()
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

        fn consume_stack(&mut self) -> String {
            let consumed = self.stack.clone();
            self.stack.clear();
            consumed 
        }

        pub fn accept_token(&mut self, kind: TokenKind) {
            let t = Token::new(kind, self.consume_stack(), self.start);
            self.tokens.push(t);
            self.start = self.current;
        }

        pub fn get_token(&mut self) -> Option<Token> {
            self.tokens.pop()
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
}

#[cfg(test)]
mod tests {
    use crate::util::Location;
    use crate::tokens::{Token, TokenKind, Op, Action};
    use super::Lexer;

    fn token_string_literal(string: &str, location: (usize, usize)) -> Token {
        Token::new(
            TokenKind::StringLiteral,
            string.to_string(),
            Location::from(location),
        )
    }

    #[test]
    fn lexer_from_str() {
        let expected = "This is a test";
        let lexer = Lexer::from(expected);

        let actual = lexer.stack.as_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_from_string() {
        let expected = String::from("This is a test y̆ou need");
        let lexer = Lexer::from(expected.clone());

        let actual = lexer.stack.as_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lexer_block_open() {
        let input = "test {{";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("test ", (0, 0)),
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0, 5)
            )
        ];

        assert_eq!(tokens, expected);
    }   

    #[test]
    fn lexer_block_open_trick() {
        let input = "this is { a test {{{";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("this is { a test ",  (0, 0)),
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0, 17)
            ), 
            token_string_literal("{",  (0, 19)),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_invalid_block_close() {
        let input = "This is a not a closed block }, and neither is this }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("This is a not a closed block }, and neither is this ",  (0, 0)),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0, 52)
            ), 
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_open_close() {
        let input = "{{}}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0),
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,2)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_digit() {
        let input = "{{ 1234 }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::NumberLiteral(1234),
                String::from("1234"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,8)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer<DIGIT>: The expected number contains invalid digit \'a\' with stack \"1234\". On line [0; 7]:\n\t{{ 1234a }}\n\t       ^ expected digit")]
    fn lexer_block_invalid_digit() {
        let input = "{{ 1234a }}";
        let lexer = Lexer::from(input);

        lexer.tokenize();
    }

    
    #[test]
    fn lexer_block_label() {
        let input = "{{label}}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("label"),
                Location::new(0,2)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,7)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer<LABEL>: The expected label contains digit \'1\' with stack \"b\". On line [0; 4]:\n\t{{ b1234 }}\n\t    ^ expected alphabetic character")]
    fn lexer_block_invalid_label() {
        let input = "{{ b1234 }}";
        let lexer = Lexer::from(input);

        lexer.tokenize();
    }

    #[test]
    fn lexer_block_string_literal() {
        let input = "{{ \"string \\\" literal\" }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::StringLiteral,
                String::from("string \" literal"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,23)
            ),
        ];

        assert_eq!(tokens, expected);
    }


    #[test]
    fn lexer_block_action() {
        let input = "{{ render! }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Action(Action::Render),
                String::from("render!"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,11)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_and() {
        let input = "{{ 1 && test && 3 }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::NumberLiteral(1),
                String::from("1"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::And),
                String::from("&&"),
                Location::new(0,5)
            ),
            Token::new(
                TokenKind::Label,
                String::from("test"),
                Location::new(0,8)
            ),
            Token::new(
                TokenKind::Operator(Op::And),
                String::from("&&"),
                Location::new(0,13)
            ),
            Token::new(
                TokenKind::NumberLiteral(3),
                String::from("3"),
                Location::new(0,16)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,18)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_assign() {
        let input = "{{ pages = site }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("pages"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Assign),
                String::from("="),
                Location::new(0,9)
            ),
            Token::new(
                TokenKind::Label,
                String::from("site"),
                Location::new(0,11)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,16)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_comma() {
        let input = "{{ test, \"test\", 2 }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("test"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Comma),
                String::from(","),
                Location::new(0,7)
            ),
            Token::new(
                TokenKind::StringLiteral,
                String::from("test"),
                Location::new(0,9)
            ),
            Token::new(
                TokenKind::Operator(Op::Comma),
                String::from(","),
                Location::new(0,15)
            ),
            Token::new(
                TokenKind::NumberLiteral(2),
                String::from("2"),
                Location::new(0,17)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,19)
            ),
        ];

        assert_eq!(tokens, expected);
    }
    
    #[test]
    fn lexer_block_closure() {
        let input = "{{ [self.album] }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Operator(Op::ClosureOpen),
                String::from("["),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Label,
                String::from("self"),
                Location::new(0,4)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                Location::new(0,8)
            ),
            Token::new(
                TokenKind::Label,
                String::from("album"),
                Location::new(0,9)
            ),
            Token::new(
                TokenKind::Operator(Op::ClosureClose),
                String::from("]"),
                Location::new(0,14)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,16)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_dot() {
        let input = "{{ site.posts }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("site"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                Location::new(0,7)
            ),
            Token::new(
                TokenKind::Label,
                String::from("posts"),
                Location::new(0,8)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,14)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_each() {
        let input = "{{ item : items }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("item"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Each),
                String::from(":"),
                Location::new(0,8)
            ),
            Token::new(
                TokenKind::Label,
                String::from("items"),
                Location::new(0,10)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,16)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_equality() {
        let input = "{{ falsy = 1 == 2 }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("falsy"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Assign),
                String::from("="),
                Location::new(0,9)
            ),
            Token::new(
                TokenKind::NumberLiteral(1),
                String::from("1"),
                Location::new(0,11)
            ),
            Token::new(
                TokenKind::Operator(Op::Equality),
                String::from("=="),
                Location::new(0,13)
            ),
            Token::new(
                TokenKind::NumberLiteral(2),
                String::from("2"),
                Location::new(0,16)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,18)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_inequality() {
        let input = "{{ truthy = 1 != 2 }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("truthy"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Assign),
                String::from("="),
                Location::new(0,10)
            ),
            Token::new(
                TokenKind::NumberLiteral(1),
                String::from("1"),
                Location::new(0,12)
            ),
            Token::new(
                TokenKind::Operator(Op::Inequality),
                String::from("!="),
                Location::new(0,14)
            ),
            Token::new(
                TokenKind::NumberLiteral(2),
                String::from("2"),
                Location::new(0,17)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,19)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_block_or() {
        let input = "{{ 1 || test || 3 }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::NumberLiteral(1),
                String::from("1"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Or),
                String::from("||"),
                Location::new(0,5)
            ),
            Token::new(
                TokenKind::Label,
                String::from("test"),
                Location::new(0,8)
            ),
            Token::new(
                TokenKind::Operator(Op::Or),
                String::from("||"),
                Location::new(0,13)
            ),
            Token::new(
                TokenKind::NumberLiteral(3),
                String::from("3"),
                Location::new(0,16)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,18)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer<BLOCK>: Encountered unknown character \'`\'. On line [1; 3]:\n\t{{ `` }}\n\t   ^ unknown character")]
    fn lexer_block_unknown_character() {
        let input = "test\n{{ `` }}\ntest";
        let lexer = Lexer::from(input);

        lexer.tokenize();
    }

    #[test]
    fn lexer_block_pipe() {
        let input = "{{ posts | reverse | take }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("posts"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Pipe),
                String::from("|"),
                Location::new(0,9)
            ),
            Token::new(
                TokenKind::Label,
                String::from("reverse"),
                Location::new(0,11)
            ),
            Token::new(
                TokenKind::Operator(Op::Pipe),
                String::from("|"),
                Location::new(0,19)
            ),
            Token::new(
                TokenKind::Label,
                String::from("take"),
                Location::new(0,21)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(0,26)
            ),
        ];

        assert_eq!(tokens, expected);
    }
    

    #[test]
    fn lexer_simple() {
        let input = "<h1>Tests</h1>\n{{ render! tests : site.tests | take 5 }}\n<li>{{ tests.name }}</li>\n{{!}}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("<h1>Tests</h1>\n",  (0, 0)),
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(1,0)
            ),
            Token::new(
                TokenKind::Action(Action::Render),
                String::from("render!"),
                Location::new(1,3)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                Location::new(1,11)
            ),
            Token::new(
                TokenKind::Operator(Op::Each),
                String::from(":"),
                Location::new(1,17)
            ),
            Token::new(
                TokenKind::Label,
                String::from("site"),
                Location::new(1,19)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                Location::new(1,23)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                Location::new(1,24)
            ),
            Token::new(
                TokenKind::Operator(Op::Pipe),
                String::from("|"),
                Location::new(1,30)
            ),
            Token::new(
                TokenKind::Label,
                String::from("take"),
                Location::new(1,32)
            ),
            Token::new(
                TokenKind::NumberLiteral(5),
                String::from("5"),
                Location::new(1,37)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(1,39)
            ),
            token_string_literal("\n<li>", (1, 41)),
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(2,4)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                Location::new(2,7)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                Location::new(2,12)
            ),
            Token::new(
                TokenKind::Label,
                String::from("name"),
                Location::new(2,13)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(2,18)
            ),
            token_string_literal("</li>\n", (2, 20)),
            Token::new(
                TokenKind::Operator(Op::BlockOpen),
                String::from("{{"),
                Location::new(3,0)
            ),
            Token::new(
                TokenKind::Action(Action::End),
                String::from("!"),
                Location::new(3,2)
            ),
            Token::new(
                TokenKind::Operator(Op::BlockClose),
                String::from("}}"),
                Location::new(3,3)
            ),
        ];

        assert_eq!(tokens, expected);
    }
} 