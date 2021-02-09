// We don't use a state transition table (2D Array) because I want to support multibyte characters correctly, 
//  and that would blow up the dimensions of the table

mod automata {
    use crate::tokens::{
        TokenKind,
        Token,
        Op,
    };
    

    type State = &'static States;
    type TransitionFn = fn(&mut PushdownAutomata, char); // Newtype pattern so we can implement debug trait 

    pub struct PushdownAutomata {
        state: State,
        stack: String,
        cursor: usize,
        location_start: (usize, usize),
        location_curr: (usize, usize),
        tokens: Vec<Token>,
    }

    impl PushdownAutomata {
        pub fn new() -> PushdownAutomata {
            PushdownAutomata {
                state: States::new(),
                stack: String::new(),
                cursor: 0,
                location_start: (0, 0),
                location_curr: (0, 0),
                tokens: Vec::new(),
            }
        }

        pub fn run(mut self, input: &str) -> Vec<Token> {
            let input_tape: Vec<char> = input.chars().collect();
            while self.cursor < input_tape.len() {
                let ch = input_tape[self.cursor];
                match ch {
                    '\n' => {
                        self.state.run(&mut self, ch);
                        self.newline();
                    },
                    _ => {
                        self.state.run(&mut self, ch);
                    }
                }
            }

            // consume leftovers 
            if self.stack.len() > 0 {
                self.consume(TokenKind::StringLiteral);
            }

            self.tokens
        }

        // ignore/skip the character (e.g. ignore whitespace )
        fn skip(&mut self) {
            self.cursor += 1;
            self.location_start.1 += 1;
            self.location_curr.1 += 1;
        }

        fn push(&mut self, ch: char) {
            self.stack.push(ch);
            self.cursor += 1;
            self.location_curr.1 += 1;
        }

        fn consume(&mut self, kind: TokenKind) {
            let token = Token::new(kind, self.stack.clone(), self.get_location());
            self.stack.clear();
            self.tokens.push(token);
        }

        fn transition(&mut self, s: State) {
            self.state = s;
        }

        #[inline]
        fn newline(&mut self) {
            self.location_curr.0 += 1;
            self.location_curr.1 = 0;
        }

        #[inline]
        fn get_location(&mut self) -> (usize, usize) {
            let loc = self.location_start;
            self.location_start = self.location_curr;

            loc
        }
    }

    enum States {
        Transition(TransitionFn),
    }

    impl States {
        pub fn new() -> State {
            &STATE_START
        }

        pub fn run(&'static self, pda: &mut PushdownAutomata, ch: char) {
            match self {
                States::Transition(transition) => transition(pda, ch)
            }
        }
    }

    static STATE_START: States = States::Transition(
        |pda, ch| {
            println!("STATE_START({})", ch);

            match ch {
                '{' => {
                    // Don't push character to stack because we don't know if this is a single '{' or the operator '{{' 
                    // Avoids consuming an empty string literal if we run on the input '{{{'
                    if pda.stack.len() > 0 {
                        pda.consume(TokenKind::StringLiteral);
                    }
                    pda.push(ch);
                    pda.transition(&STATE_OPEN_BLOCK_OP);
                },
                _ => pda.push(ch)
            }
        }
    );

    static STATE_OPEN_BLOCK_OP: States = States::Transition(
        |pda, ch| {
            println!("STATE_OPEN_BLOCK_OP({})", ch);

            pda.push(ch);
            match ch {
                '{' => {
                    pda.consume(TokenKind::Operator(Op::OpenBlock));
                    pda.transition(&STATE_EXPRESSION);
                },
                _ => {
                    pda.transition(&STATE_START);
                }
            }
        }
    );
    
    static STATE_EXPRESSION: States = States::Transition(
        |pda, ch| {
            println!("STATE_EXPRESSION({})", ch);

            match ch {
                '!' => {
                    if pda.stack.len() > 0 {
                        pda.consume(TokenKind::Label);
                    }
                    pda.push(ch);
                    pda.consume(TokenKind::Operator(Op::Action));
                },
                '=' => {
                    if pda.stack.len() > 0 {
                        pda.consume(TokenKind::Label);
                    }
                    pda.push(ch);
                    pda.consume(TokenKind::Operator(Op::Assign));
                },
                '|' => {
                    if pda.stack.len() > 0 {
                        pda.consume(TokenKind::Label);
                    }
                    pda.push(ch);
                    pda.consume(TokenKind::Operator(Op::Pipe));
                }
                '.' => {
                    if pda.stack.len() > 0 {
                        pda.consume(TokenKind::Label);
                    }
                    pda.push(ch);
                    pda.consume(TokenKind::Operator(Op::Dot));
                }
                '}' => {
                    if pda.stack.len() > 0 {
                        pda.consume(TokenKind::Label);
                    }
                    pda.push(ch);
                    pda.transition(&STATE_CLOSE_BLOCK_OP);
                },
                _ => {
                    if ch.is_whitespace() {
                        // Consume label if there is content in the stack 
                        if pda.stack.len() > 0 {
                            pda.consume(TokenKind::Label);
                        }
                        pda.skip()
                    } else if ch.is_numeric() {
                        if pda.stack.len() == 0 {
                            pda.push(ch);
                            pda.transition(&STATE_DIGIT);
                        } else {
                            // Labels cannot contain a digit 
                            panic!("Lexer: expected label contains digit '{}' with stack \"{}\"", ch, pda.stack);
                        }
                    } else {
                        pda.push(ch);
                    }
                }
            }
        }
    );

    static STATE_DIGIT: States = States::Transition(
        |pda, ch| {
            println!("STATE_DIGIT({})", ch);

            if ch.is_numeric() {
                pda.push(ch);
            } else if ch.is_alphabetic() {
                // We don't accept letters with our digits (not even for labels)
                panic!("Lexer: expected number contains invalid digit '{}' with stack \"{}\"", ch, pda.stack);
            } else {
                let number = pda.stack.parse::<usize>().unwrap();
                pda.consume(TokenKind::NumberLiteral(number));
                pda.transition(&STATE_EXPRESSION);
            }
        }
    );

    static STATE_CLOSE_BLOCK_OP: States = States::Transition(
        |pda, ch| {
            println!("STATE_CLOSE_BLOCK_OP({})", ch);

            pda.push(ch);
            match ch {
                '}' => {
                    pda.consume(TokenKind::Operator(Op::CloseBlock));
                    pda.transition(&STATE_START);
                },
                _ => {
                    pda.transition(&STATE_EXPRESSION);
                }
            }
        }
    );
}

mod lexer {
    use std::fs::File;
    use std::str::Chars;
    use std::convert;
    use std::io::Read;

    use crate::tokens::Token;
    use super::automata::PushdownAutomata;

    pub struct Lexer {
        input: String,
        pda: PushdownAutomata
    }

    impl Lexer {
        pub fn chars(&self) -> Chars {
            self.input.chars()
        }
    }

    // Lexical Analysis 
    impl Lexer {
        pub fn tokenize(self) -> Vec<Token> {
            let tokens = self.pda.run(&self.input);

            tokens
        }
    }

    impl convert::From<&str> for Lexer {
        fn from(string: &str) -> Lexer {
            Lexer {
                input: string.to_string(),
                pda: PushdownAutomata::new(),
            }
        }
    }

    impl convert::From<String> for Lexer {
        fn from(string: String) -> Lexer {
            Lexer {
                input: string,
                pda: PushdownAutomata::new(),
            }
        }
    }

    impl convert::From<File> for Lexer {
        fn from(mut file: File) -> Lexer {
            let mut input = String::with_capacity(4096); 
            
            let result = file.read_to_string(&mut input);

            match result {
                Ok(_) => {
                    Lexer {
                        input,
                        pda: PushdownAutomata::new(),
                    }
                },
                Err(error) => {
                    panic!("<lexer::from<File>()> failed to read contents of file into a string:\n{}", error);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens::{Token, TokenKind, Op};
    use super::lexer::Lexer;

    fn token_string_literal(string: &str, location: (usize, usize)) -> Token {
        Token::new(
            TokenKind::StringLiteral,
            string.to_string(),
            location,
        )
    }

    #[test]
    fn lexer_from_str() {
        let string = "This is a test";
        let lexer = Lexer::from(string);

        let mut consumer = String::with_capacity(string.len());

        for char in lexer.chars() {
            consumer.push(char)
        }

        assert_eq!(string, consumer);
    }

    #[test]
    fn lexer_from_string() {
        let string = String::from("This is a test y̆ou need");
        let lexer = Lexer::from(string.clone());


        let mut consumer = String::with_capacity(string.len());

        for char in lexer.chars() {
            consumer.push(char);
        }

        assert_eq!(string, consumer);
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
                (0, 5)
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
            token_string_literal("this is ", (0, 0)),
            token_string_literal("{ a test ", (0, 8)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                (0, 17)
            ), 
            token_string_literal("{", (0, 19)),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn lexer_invalid_close_block() {
        let input = "This is a not a closed block }, and neither is this }}";
        let lexer = Lexer::from(input);

        let tokens = lexer.tokenize();

        let expected = vec![
            token_string_literal("This is a not a closed block }, and neither is this }}", (0, 0)),
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
                (0,0)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (0,2)
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
                (0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("render"),
                (0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Action),
                String::from("!"),
                (0,9)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (0,11)
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
                (0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("pages"),
                (0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Assign),
                String::from("="),
                (0,9)
            ),
            Token::new(
                TokenKind::Label,
                String::from("site"),
                (0,11)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (0,16)
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
                (0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("site"),
                (0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                (0,7)
            ),
            Token::new(
                TokenKind::Label,
                String::from("posts"),
                (0,8)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (0,14)
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
                (0,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("posts"),
                (0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Pipe),
                String::from("|"),
                (0,9)
            ),
            Token::new(
                TokenKind::Label,
                String::from("reverse"),
                (0,11)
            ),
            Token::new(
                TokenKind::Operator(Op::Pipe),
                String::from("|"),
                (0,19)
            ),
            Token::new(
                TokenKind::Label,
                String::from("take"),
                (0,21)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (0,26)
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
                (0,0)
            ),
            Token::new(
                TokenKind::NumberLiteral(1234),
                String::from("1234"),
                (0,3)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (0,8)
            ),
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer: expected number contains invalid digit 'a' with stack \"1234\"")]
    fn lexer_expr_invalid_digit() {
        let input = "{{ 1234a }}";
        let lexer = Lexer::from(input);

        lexer.tokenize();
    }

    #[test]
    #[should_panic(expected = "Lexer: expected label contains digit '1' with stack \"b\"")]
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
            token_string_literal("<h1>Tests</h1>\n", (0, 0)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                (1,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("render"),
                (1,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Action),
                String::from("!"),
                (1,9)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (1,11)
            ),
            token_string_literal("\n", (1, 13)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                (2,0)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                (2,3)
            ),
            Token::new(
                TokenKind::Operator(Op::Assign),
                String::from("="),
                (2,9)
            ),
            Token::new(
                TokenKind::Label,
                String::from("site"),
                (2,11)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                (2,15)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                (2,16)
            ),
            Token::new(
                TokenKind::Operator(Op::Pipe),
                String::from("|"),
                (2,22)
            ),
            Token::new(
                TokenKind::Label,
                String::from("take"),
                (2,24)
            ),
            Token::new(
                TokenKind::NumberLiteral(5),
                String::from("5"),
                (2,29)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (2,31)
            ),
            token_string_literal("\n<li>", (2, 33)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                (3,4)
            ),
            Token::new(
                TokenKind::Label,
                String::from("tests"),
                (3,7)
            ),
            Token::new(
                TokenKind::Operator(Op::Dot),
                String::from("."),
                (3,12)
            ),
            Token::new(
                TokenKind::Label,
                String::from("name"),
                (3,13)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (3,18)
            ),
            token_string_literal("</li>\n", (3, 20)),
            Token::new(
                TokenKind::Operator(Op::OpenBlock),
                String::from("{{"),
                (4,0)
            ),
            Token::new(
                TokenKind::Operator(Op::Action),
                String::from("!"),
                (4,2)
            ),
            Token::new(
                TokenKind::Operator(Op::CloseBlock),
                String::from("}}"),
                (4,3)
            ),
        ];

        assert_eq!(tokens, expected);
    }
}