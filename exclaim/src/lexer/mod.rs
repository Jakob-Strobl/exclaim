use crate::tokens::Token;

pub mod automata;
use automata::StackMachine;

pub mod tokens;
use tokens::*;

pub mod tests;

pub fn run<S: AsRef<str>>(input: S) -> Result<Vec<Token>, String> {
    let mut state = State::new();
    let mut stack = StackMachine::new(input);

    while !stack.eof() {
        if stack.peek() == '\n' {
            state = state.run(&mut stack)?;
            stack.newline();
        } else {
            state = state.run(&mut stack)?;
        }
    }

    // consume leftovers
    if !stack.empty() {
        stack.accept_token(Token::StringLiteral(stack.view_stack().to_string(), stack.location()));
    }

    Ok(stack.get_tokens())
}

struct State(fn(&mut StackMachine) -> Result<&'static State, String>);

impl State {
    pub fn new() -> &'static State {
        &STATE_START
    }

    pub fn run(&self, stack: &mut StackMachine) -> Result<&'static State, String> {
        self.0(stack)
    }

    pub fn get_error_msg(stack: &mut StackMachine, msg: &str, underline_msg: &str) -> String {
        let (loc, line) = stack.debug_line(underline_msg);
        format!("{} On line [{}; {}]:\n\t{}", msg, loc.line(), loc.column(), line)
    }
}

static STATE_START: State = State(
    |stack| {
        match stack.peek() {
            '{' => Ok(&STATE_OPEN_BLOCK),
            '}' => Ok(&STATE_CLOSE_BLOCK),
            _ => {
                stack.push();
                Ok(&STATE_START)
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
                    stack.accept_token(Token::StringLiteral(stack.view_stack().to_string(), stack.location()));
                }
                Ok(&ACCEPT_OPEN_BLOCK)
            },
            _ => {
                stack.push();
                Ok(&STATE_START)
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
                    stack.accept_token(Token::StringLiteral(stack.view_stack().to_string(), stack.location()));
                }
                Ok(&ACCEPT_CLOSE_BLOCK)
            },
            _ => {
                stack.push();
                Ok(&STATE_START)
            }
        }
    }
);

static ACCEPT_OPEN_BLOCK: State = State(
    |stack| {
        stack.push(); // {
        stack.push(); // {{
        stack.accept_token(Token::Operator(Op::BlockOpen, stack.location()));
        Ok(&STATE_BLOCK)
    }
);

static ACCEPT_CLOSE_BLOCK: State = State(
    |stack| {
        stack.push(); // }
        stack.push(); // }}
        stack.accept_token(Token::Operator(Op::BlockClose, stack.location()));
        Ok(&STATE_START)
    }
);

static STATE_BLOCK: State = State(
    |stack| {
        let ch = stack.peek();
        match ch {
            '{' => Ok(&STATE_OPEN_BLOCK_FROM_BLOCK),
            '}' => Ok(&STATE_CLOSE_BLOCK_FROM_BLOCK),
            '!' => Ok(&STATE_BLOCK_ACTION_INEQUALITY),
            '=' => Ok(&STATE_BLOCK_ASSIGN_EQUALITY),
            '|' => Ok(&STATE_BLOCK_PIPE_OR),
            '&' => Ok(&STATE_BLOCK_AND),
            ',' => {
                stack.push();
                stack.accept_token(Token::Operator(Op::Comma, stack.location()));
                Ok(&STATE_BLOCK)
            }
            '.' => {
                stack.push();
                stack.accept_token(Token::Operator(Op::Dot, stack.location()));
                Ok(&STATE_BLOCK)
            },
            ':' => {
                stack.push();
                stack.accept_token(Token::Operator(Op::Each, stack.location()));
                Ok(&STATE_BLOCK)
            },
            '"' => {
                stack.skip_current();
                Ok(&STATE_BLOCK_STRING_LITERAL)
            },
            '[' => {
                stack.push();
                stack.accept_token(Token::Operator(Op::ClosureOpen, stack.location()));
                Ok(&STATE_BLOCK)
            },
            ']' => {
                stack.push();
                stack.accept_token(Token::Operator(Op::ClosureClose, stack.location()));
                Ok(&STATE_BLOCK)
            },
            '(' => {
                stack.push();
                stack.accept_token(Token::Operator(Op::ParenOpen, stack.location()));
                Ok(&STATE_BLOCK)
            },
            ')' => {
                stack.push();
                stack.accept_token(Token::Operator(Op::ParenClose, stack.location()));
                Ok(&STATE_BLOCK)
            },
            _ => {
                if ch.is_alphabetic() {
                    stack.push();
                    Ok(&STATE_LABEL_ACTION)
                } else if ch.is_numeric() {
                    stack.push();
                    Ok(&STATE_DIGIT)
                } else if ch.is_whitespace() {
                    stack.skip();
                    Ok(&STATE_BLOCK)
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
                    stack.accept_token(Token::StringLiteral(stack.view_stack().to_string(), stack.location()));
                }
                Ok(&ACCEPT_OPEN_BLOCK)
            },
            _ => {
                stack.push();
                Ok(&STATE_BLOCK)
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
                    stack.accept_token(Token::StringLiteral(stack.view_stack().to_string(), stack.location()));
                }
                Ok(&ACCEPT_CLOSE_BLOCK)
            },
            _ => {
                stack.push();
                Ok(&STATE_BLOCK)
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
                stack.accept_token(Token::Operator(Op::Inequality, stack.location()));
            }
            _ => {
                stack.push();
                stack.accept_token(Token::Action(Action::End, stack.location()));
            }
        }

        Ok(&STATE_BLOCK)
    }
);


static STATE_BLOCK_ASSIGN_EQUALITY: State = State(
    |stack| {
        // Context, we already know stack.peek() == '='
        match stack.lookahead().unwrap_or(&' ') {
            '=' => {
                stack.push(); // =
                stack.push(); // ==
                stack.accept_token(Token::Operator(Op::Equality, stack.location()));
            }
            _ => {
                stack.push(); // =
                stack.accept_token(Token::Operator(Op::Assign, stack.location()));
            }
        }

        Ok(&STATE_BLOCK)
    }
);

static STATE_BLOCK_STRING_LITERAL: State = State(
    |stack| {
        match stack.peek() {
            '"' => { 
                stack.skip_current();   // Skip closing double quote
                stack.accept_token(Token::StringLiteral(stack.view_stack().to_string(), stack.location()));
                Ok(&STATE_BLOCK)
            },
            '\\' => { // ESCAPE CHARACTER
                stack.skip_current();   // Skip escape
                stack.push();           // Push character escaped
                Ok(&STATE_BLOCK_STRING_LITERAL)
            },
            _ => {
                stack.push();
                Ok(&STATE_BLOCK_STRING_LITERAL)
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
                stack.accept_token(Token::Operator(Op::And, stack.location()));
                Ok(&STATE_BLOCK)
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
                stack.accept_token(Token::Operator(Op::Or, stack.location()));
            }
            _ => {
                stack.push(); // | Pipe
                stack.accept_token(Token::Operator(Op::Pipe, stack.location()));
            }
        }
        Ok(&STATE_BLOCK)
    }
);

static STATE_LABEL_ACTION: State = State(
    |stack| {
        let ch = stack.peek();
        if ch.is_alphabetic() || ch == '_' {
            stack.push();
            Ok(&STATE_LABEL_ACTION)
        } else if ch == '!' && *stack.lookahead().unwrap_or(&' ') != '=' {
            // If the following two characters are not: !=
            // Push ! 
            stack.push();

            // Get action variant
            let action = match stack.view_stack() {
                // There is no way it should be an empty string, since one character has to be consumed to even be in this state.
                "let!" => Action::Let,
                "write!" => Action::Write,
                "render!" => Action::Render,
                _ => panic!(State::get_error_msg(
                                stack, 
                                &format!("Lexer<LABEL>: The expected action does not match any defined action - invalid action found: '{}' ", stack.view_stack()), 
                                "expected one of the following defined actions: let!, write!, render!, or !."))
            };

            stack.accept_token(Token::Action(action, stack.location()));
            Ok(&STATE_BLOCK)

        } else if ch.is_numeric() {
            panic!(State::get_error_msg(
                stack, 
                &format!("Lexer<LABEL>: The expected label contains digit '{}' with stack \"{}\".", ch, stack.view_stack()),
                "expected alphabetic character",
            ));
        } else {
            // Accept Label 
            stack.accept_token(Token::Label(stack.view_stack().to_string(), stack.location()));
            Ok(&STATE_BLOCK)
        }
    }
);

static STATE_DIGIT: State = State(
    |stack| {
        let ch = stack.peek();
        if ch.is_numeric() {
            stack.push();
            Ok(&STATE_DIGIT)
        } else if ch.is_alphabetic() {
            panic!(State::get_error_msg(
                stack, 
                &format!("Lexer<DIGIT>: The expected number contains invalid digit '{}' with stack \"{}\".", ch, stack.view_stack()),
                "expected digit",
            ));
        } else {
            // Accept Number 
            let number: usize = stack.view_stack().parse::<usize>().unwrap();
            let token = Token::NumberLiteral(number, stack.location());
            stack.accept_token(token);
            Ok(&STATE_BLOCK)
        }
    }
);