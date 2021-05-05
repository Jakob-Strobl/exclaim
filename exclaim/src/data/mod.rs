use std::collections::HashMap;

use crate::ast::expressions::Transform;
use crate::lexer::tokens::Token;

pub mod traits;
use traits::Renderable;

pub mod transforms;
use transforms::apply_transform;

pub enum Data {
    // Reserved - value not initialized
    Any, 

    // Scalar
    String(String),
    Int(isize),
    Uint(usize),
    Float(f64),

    // Compound Types
    Tuple(Box<[Data]>),
    Object(HashMap<String, Data>),
    Array(Vec<Data>),
}

impl Data {
    pub fn apply_transform(self, transform: &Transform) -> Data {
        apply_transform(self, transform)
    }

    pub fn is_scalar(&self) -> bool {
        match self {
            Data::String(_) => true,
            Data::Int(_) => true,
            Data::Uint(_) => true,
            Data::Float(_) => true,
            _ => false,
        }
    }
}

impl From<Token> for Data {
    fn from(token: Token) -> Self {
        match token {
            Token::StringLiteral(string, _) => Data::String(string),
            Token::NumberLiteral(number, _) => Data::Uint(number),
            _ => panic!("Cannot convert token into Data: {:?}", token),
        }
    }
}

impl Renderable for Data {
    fn render(&self) -> String {
        match self {
            Data::Any => panic!("DataType::Any can not be rendered!"),
            Data::String(s) => s.to_string(),
            Data::Int(num) => num.to_string(),
            Data::Uint(num) => num.to_string(),
            Data::Float(num) => num.to_string(),
            Data::Tuple(_) => panic!("DataType::Tuple unimplemented!"),
            Data::Object(_) => panic!("DataType::Object unimplemented!"),
            Data::Array(_) => panic!("DataType::Array unimplemented!"),
        }
    }
}

pub struct DataContext {
    data: HashMap<String, Data>,
}

impl DataContext {
    pub fn new() -> DataContext {
        DataContext {
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Data) -> Option<Data> {
        self.data.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&Data> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Data> {
        self.data.get_mut(key)
    }
}