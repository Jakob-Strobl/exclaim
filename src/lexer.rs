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
    use crate::tokens::{TokenKind, Op};
    
    pub struct State(fn(&mut StackMachine) -> &'static State);

    impl State {
        pub fn new() -> &'static State {
            &STATE_START
        }

        pub fn run(&self, stack: &mut StackMachine) -> &'static State {
            &self.0(stack)
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
            match stack.lookahead().unwrap_or(&' ') {
                '{' => {
                    // Accept string literal if the stack is not empty, because next token is a OpenBlock 
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
            match stack.lookahead().unwrap() {
                '}' => {
                    // Accept string literal if the stack is not empty, because next token is a CloseBlock 
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
            stack.accept_token(TokenKind::Operator(Op::OpenBlock));
            &STATE_BLOCK
        }
    );

    static ACCEPT_CLOSE_BLOCK: State = State(
        |stack| {
            stack.push(); // }
            stack.push(); // }}
            stack.accept_token(TokenKind::Operator(Op::CloseBlock));
            &STATE_START
        }
    );

    static STATE_BLOCK: State = State(
        |stack| {
            let ch = stack.peek();
            match ch {
                '{' => &STATE_OPEN_BLOCK_FROM_BLOCK,
                '}' => &STATE_CLOSE_BLOCK_FROM_BLOCK,
                '!' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::Action));
                    &STATE_BLOCK
                },
                '=' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::Assign));
                    &STATE_BLOCK
                },
                '.' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::Dot));
                    &STATE_BLOCK
                },
                '|' => {
                    stack.push();
                    stack.accept_token(TokenKind::Operator(Op::Pipe));
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
                        panic!("Lexer: Encountered unknown character '{}'", ch);
                    }
                }
            }
        }
    );

    static STATE_OPEN_BLOCK_FROM_BLOCK: State = State(
        |stack| {
            match stack.lookahead().unwrap_or(&' ') {
                '{' => {
                    // Accept string literal if the stack is not empty, because next token is a OpenBlock 
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
            match stack.lookahead().unwrap() {
                '}' => {
                    // Accept string literal if the stack is not empty, because next token is a CloseBlock 
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

    static STATE_LABEL: State = State(
        |stack| {
            let ch = stack.peek();
            if ch.is_alphabetic() {
                stack.push();
                &STATE_LABEL
            } else if ch.is_numeric() {
                panic!("Lexer: The expected label contains digit '{}' with stack \"{}\"", ch, stack.view_stack());
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
                panic!("Lexer: The expected number contains invalid digit '{}' with stack \"{}\"", ch, stack.view_stack());
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
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Location;
    use crate::tokens::{Token, TokenKind, Op};
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
    fn lexer_open_block() {
        let input = "test {{";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("test ", (0, 0)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(0, 5)
            )
        ];

        assert_eq!(tokens, expected);
    }   

    #[test]
    fn lexer_open_block_trick() {
        let input = "this is { a test {{{";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("this is { a test ",  (0, 0)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(0, 17)
            ), 
            token_string_literal("{",  (0, 19)),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_invalid_close_block() {
        let input = "This is a not a closed block }, and neither is this }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("This is a not a closed block }, and neither is this ",  (0, 0)),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(0, 52)
            ), 
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_open_close_block() {
        let input = "{{}}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(0,0),
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(0,2)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_expr_action() {
        let input = "{{ render! }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("render"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Action),
                String::from("!"),
                Location::new(0,9)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(0,11)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_expr_assign() {
        let input = "{{ pages = site }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
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
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(0,16)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_expr_dot() {
        let input = "{{ site.posts }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
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
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(0,14)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_expr_pipe() {
        let input = "{{ posts | reverse | take }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
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
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(0,26)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_expr_digit() {
        let input = "{{ 1234 }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(0,0)
            ),
            Token::new(
                TokenKind::NumberLiteral(1234),
                String::from("1234"),
                Location::new(0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(0,8)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer: The expected number contains invalid digit 'a' with stack \"1234\"")]
    fn lexer_expr_invalid_digit() {
        let input = "{{ 1234a }}";
        let lexer = Lexer::from(input);

        lexer.tokenize();
    }

    #[test]
    #[should_panic(expected = "Lexer: The expected label contains digit '1' with stack \"b\"")]
    fn lexer_expr_invalid_label() {
        let input = "{{ b1234 }}";
        let lexer = Lexer::from(input);

        lexer.tokenize();
    }

    #[test]
    fn lexer_simple() {
        let input = "<h1>Tests</h1>\n{{ render! }}\n{{ tests = site.tests | take 5 }}\n<li>{{ tests.name }}</li>\n{{!}}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("<h1>Tests</h1>\n",  (0, 0)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(1,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("render"),
                Location::new(1,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Action),
                String::from("!"),
                Location::new(1,9)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(1,11)
            ),
            token_string_literal("\n", (1, 13)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(2,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                Location::new(2,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Assign),
                String::from("="),
                Location::new(2,9)
            ),
            Token::new(
                TokenKind::Label,
                String::from("site"),
                Location::new(2,11)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                Location::new(2,15)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                Location::new(2,16)
            ),
            Token::new(
                TokenKind::Operator(Op::Pipe),
                String::from("|"),
                Location::new(2,22)
            ),
            Token::new(
                TokenKind::Label,
                String::from("take"),
                Location::new(2,24)
            ),
            Token::new(
                TokenKind::NumberLiteral(5),
                String::from("5"),
                Location::new(2,29)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(2,31)
            ),
            token_string_literal("\n<li>", (2, 33)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(3,4)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                Location::new(3,7)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                Location::new(3,12)
            ),
            Token::new(
                TokenKind::Label,
                String::from("name"),
                Location::new(3,13)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(3,18)
            ),
            token_string_literal("</li>\n", (3, 20)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                Location::new(4,0)
            ),
            Token::new(
                TokenKind::Operator(Op::Action),
                String::from("!"),
                Location::new(4,2)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                Location::new(4,3)
            ),
        ];

        assert_eq!(tokens, expected);
    }
} 