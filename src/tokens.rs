#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Literals 
    StringLiteral,
    NumberLiteral(usize),
    
    // Identifiers
    Label,
    Keyword, 

    Operator(Op),
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Action,         // !
    Assign,         // =
    CloseBlock,     // }}
    Comma,          // , Keeping here if its used in future 
    Dot,            // . (Access fields)
    OpenBlock,      // {{
    Pipe,           // | (Chain function operations)
}
#[derive(Debug, PartialEq)]
pub struct Info {
    // Line number, starting character
    location: (usize, usize),
}

impl Info {
    fn new(location: (usize, usize)) -> Info {
        Info {
            location
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    lexeme: String, 
    info: Info,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, location: (usize, usize)) -> Token {
        Token {
            kind,
            lexeme,
            info: Info::new(location),
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}