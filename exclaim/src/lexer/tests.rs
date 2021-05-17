#[cfg(test)]
mod tests {
    use crate::common::Location;
    use crate::tokens::*;
    use crate::lexer;

    fn token_string_literal(string: &str, location: (usize, usize)) -> Token {
        Token::StringLiteral(
            string.to_string(),
            Location::from(location)
        )
    }

    #[test]
    fn lexer_block_open() {
        let input = "test {{";
        let expected = vec![
            token_string_literal("test ", (0, 0)),
            Token::Operator(Op::BlockOpen, Location::new(0, 5))
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }   

    #[test]
    fn lexer_block_open_trick() {
        let input = "this is { a test {{{";
        let expected = vec![
            token_string_literal("this is { a test ",  (0, 0)),
            Token::Operator(Op::BlockOpen, Location::new(0, 17)), 
            token_string_literal("{",  (0, 19)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_invalid_block_close() {
        let input = "This is a not a closed block }, and neither is this }}";
        let expected = vec![
            token_string_literal("This is a not a closed block }, and neither is this ",  (0, 0)),
            Token::Operator(Op::BlockClose, Location::new(0, 52)), 
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_open_close() {
        let input = "{{}}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Operator(Op::BlockClose, Location::new(0,2)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_digit() {
        let input = "{{ 1234 }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::NumberLiteral(1234, Location::new(0,3)),
            Token::Operator(Op::BlockClose, Location::new(0,8)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer<DIGIT>: The expected number contains invalid digit \'a\' with stack \"1234\". On line [0; 7]:\n\t{{ 1234a }}\n\t       ^ expected digit")]
    fn lexer_block_invalid_digit() {
        let input = "{{ 1234a }}";
        let _actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };
    }

    
    #[test]
    fn lexer_block_label() {
        let input = "{{label_label}}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Label(String::from("label_label"), Location::new(0,2)),
            Token::Operator(Op::BlockClose, Location::new(0,13)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer<LABEL>: The expected label contains digit \'1\' with stack \"b\". On line [0; 4]:\n\t{{ b1234 }}\n\t    ^ expected alphabetic character")]
    fn lexer_block_invalid_label() {
        let input = "{{ b1234 }}";
        let _actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn lexer_block_string_literal() {
        let input = "{{ \"string \\\" literal\" }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::StringLiteral(String::from("string \" literal"), Location::new(0,3)),
            Token::Operator(Op::BlockClose, Location::new(0,23)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }


    #[test]
    fn lexer_block_action() {
        let input = "{{ render! }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Action(Action::Render, Location::new(0,3)),
            Token::Operator(Op::BlockClose, Location::new(0,11)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_action_or_inequality() {
        let input = "{{ render!render!=abc }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Action(Action::Render, Location::new(0,3)),
            Token::Label(String::from("render"), Location::new(0,10)),
            Token::Operator(Op::Inequality, Location::new(0,16)),
            Token::Label(String::from("abc"), Location::new(0,18)),
            Token::Operator(Op::BlockClose, Location::new(0,22)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer<LABEL>: The expected action does not match any defined action - invalid action found: \'abc!\'")]
    fn lexer_invalid_action() {
        let input = "{{ abc! }}";
        let _actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn lexer_block_and() {
        let input = "{{ 1 && test && 3 }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::NumberLiteral(1, Location::new(0,3)),
            Token::Operator(Op::And, Location::new(0,5)),
            Token::Label(String::from("test"), Location::new(0,8)),
            Token::Operator(Op::And, Location::new(0,13)),
            Token::NumberLiteral(3, Location::new(0,16)),
            Token::Operator(Op::BlockClose, Location::new(0,18)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_assign() {
        let input = "{{ pages = site }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Label(String::from("pages"), Location::new(0,3)),
            Token::Operator(Op::Assign, Location::new(0,9)),
            Token::Label(String::from("site"), Location::new(0,11)),
            Token::Operator(Op::BlockClose, Location::new(0,16)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_comma() {
        let input = "{{ test, \"test\", 2 }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Label(String::from("test"), Location::new(0,3)),
            Token::Operator(Op::Comma, Location::new(0,7)),
            Token::StringLiteral(String::from("test"), Location::new(0,9)),
            Token::Operator(Op::Comma, Location::new(0,15)),
            Token::NumberLiteral(2, Location::new(0,17)),
            Token::Operator(Op::BlockClose, Location::new(0,19)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }
    
    #[test]
    fn lexer_block_closure() {
        let input = "{{ [self.album] }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Operator(Op::ClosureOpen, Location::new(0,3)),
            Token::Label(String::from("self"), Location::new(0,4)),
            Token::Operator(Op::Dot, Location::new(0,8)),
            Token::Label(String::from("album"), Location::new(0,9)),
            Token::Operator(Op::ClosureClose, Location::new(0,14)),
            Token::Operator(Op::BlockClose, Location::new(0,16)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_dot() {
        let input = "{{ site.posts }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Label(String::from("site"), Location::new(0,3)),
            Token::Operator(Op::Dot, Location::new(0,7)),
            Token::Label(String::from("posts"), Location::new(0,8)),
            Token::Operator(Op::BlockClose, Location::new(0,14)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_each() {
        let input = "{{ item : items }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Label(String::from("item"), Location::new(0,3)),
            Token::Operator(Op::Each, Location::new(0,8)),
            Token::Label(String::from("items"), Location::new(0,10)),
            Token::Operator(Op::BlockClose, Location::new(0,16)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_equality() {
        let input = "{{ falsy = 1 == 2 }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Label(String::from("falsy"), Location::new(0,3)),
            Token::Operator(Op::Assign, Location::new(0,9)),
            Token::NumberLiteral(1, Location::new(0,11)),
            Token::Operator(Op::Equality, Location::new(0,13)),
            Token::NumberLiteral(2, Location::new(0,16)),
            Token::Operator(Op::BlockClose, Location::new(0,18)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_inequality() {
        let input = "{{ truthy = 1 != 2 }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Label(String::from("truthy"), Location::new(0,3)),
            Token::Operator(Op::Assign, Location::new(0,10)),
            Token::NumberLiteral(1, Location::new(0,12)),
            Token::Operator(Op::Inequality, Location::new(0,14)),
            Token::NumberLiteral(2, Location::new(0,17)),
            Token::Operator(Op::BlockClose, Location::new(0,19)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn lexer_block_or() {
        let input = "{{ 1 || test || 3 }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::NumberLiteral(1, Location::new(0,3)),
            Token::Operator(Op::Or, Location::new(0,5)),
            Token::Label(String::from("test"),Location::new(0,8)),
            Token::Operator(Op::Or, Location::new(0,13)),
            Token::NumberLiteral(3, Location::new(0,16)),
            Token::Operator(Op::BlockClose, Location::new(0,18)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic(expected = "Lexer<BLOCK>: Encountered unknown character \'`\'. On line [1; 3]:\n\t{{ `` }}\n\t   ^ unknown character")]
    fn lexer_block_unknown_character() {
        let input = "test\n{{ `` }}\ntest";
        let _actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };
    }

    #[test]
    fn lexer_block_pipe() {
        let input = "{{ posts | reverse | take }}";
        let expected = vec![
            Token::Operator(Op::BlockOpen, Location::new(0,0)),
            Token::Label(String::from("posts"), Location::new(0,3)),
            Token::Operator(Op::Pipe, Location::new(0,9)),
            Token::Label(String::from("reverse"),Location::new(0,11)),
            Token::Operator(Op::Pipe, Location::new(0,19)),
            Token::Label(String::from("take"), Location::new(0,21)),
            Token::Operator(Op::BlockClose, Location::new(0,26)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }
    

    #[test]
    fn lexer_simple() {
        let input = "<h1>Tests</h1>\n{{ render! tests : site.tests | take(1,5) }}\n<li>{{ tests.name }}</li>\n{{!}}";
        let expected = vec![
            token_string_literal("<h1>Tests</h1>\n",  (0, 0)),
            Token::Operator(Op::BlockOpen, Location::new(1,0)),
            Token::Action(Action::Render, Location::new(1,3)),
            Token::Label(String::from("tests"), Location::new(1,11)),
            Token::Operator(Op::Each, Location::new(1,17)),
            Token::Label(String::from("site"), Location::new(1,19)),
            Token::Operator(Op::Dot, Location::new(1,23)),
            Token::Label(String::from("tests"), Location::new(1,24)),
            Token::Operator(Op::Pipe, Location::new(1,30)),
            Token::Label(String::from("take"), Location::new(1,32)),
            Token::Operator(Op::ParenOpen, Location::new(1,36)),
            Token::NumberLiteral(1, Location::new(1,37)),
            Token::Operator(Op::Comma, Location::new(1,38)),
            Token::NumberLiteral(5, Location::new(1,39)),
            Token::Operator(Op::ParenClose, Location::new(1,40)),
            Token::Operator(Op::BlockClose, Location::new(1,42)),
            token_string_literal("\n<li>", (1, 44)),
            Token::Operator(Op::BlockOpen, Location::new(2,4)),
            Token::Label(String::from("tests"), Location::new(2,7)),
            Token::Operator(Op::Dot, Location::new(2,12)),
            Token::Label(String::from("name"), Location::new(2,13)),
            Token::Operator(Op::BlockClose, Location::new(2,18)),
            token_string_literal("</li>\n", (2, 20)),
            Token::Operator(Op::BlockOpen, Location::new(3,0)),
            Token::Action(Action::End, Location::new(3,2)),
            Token::Operator(Op::BlockClose, Location::new(3,3)),
        ];

        let actual = match lexer::run(input) {
            Ok(tokens) => tokens,
            Err(e) => panic!(e),
        };

        assert_eq!(actual, expected);
    }
}