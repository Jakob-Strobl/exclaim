use std::collections::HashMap;
use std::fmt::Debug;

use crate::ast::transforms::Transform;
use crate::lexer::tokens::Token;

pub mod traits;
use traits::Renderable;

pub mod transforms;
use transforms::apply_transform;

#[derive(Clone)]
pub enum Data {
    // Reserved - value not initialized 
    // TODO do we actually need this? 
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

impl From<Token> for Data {
    fn from(token: Token) -> Self {
        match token {
            Token::StringLiteral(string, _) => Data::String(string),
            Token::NumberLiteral(number, _) => Data::Uint(number),
            _ => panic!("Cannot convert token into Data: {:?}", token),
        }
    }
}

impl Data {
    pub fn apply_transform(self, transform: &Transform, arguments: Vec<Data>) -> Data {
        apply_transform(self, transform, arguments)
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

    pub fn len(&self) -> usize {
        match self {
            Data::Any => 0,
            Data::Tuple(tup) => tup.len(),
            Data::Array(arr) => arr.len(),
            _ => 1,
        }
    }
}

impl IntoIterator for Data {
    type Item = Data;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Data::Any => vec![].into_iter(),
            Data::Tuple(tup) => tup.to_vec().into_iter(),
            Data::Array(arr) => arr.into_iter(),
            _ => vec![self].into_iter(),
        }
    }
}

impl Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Any => write!(f, "Data::Any"),
            Data::String(string) => write!(f, "\"{}\"", string),
            Data::Int(num) => write!(f, "{}", num),
            Data::Uint(num) => write!(f, "{}", num),
            Data::Float(num) => write!(f, "{}", num),
            Data::Tuple(tuple) => write!(f, "{:?}", tuple),
            Data::Object(object) => write!(f, "{:?}", object),
            Data::Array(array) => write!(f, "{:?}", array),
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
            Data::Array(array) => format!("{:?}", array),
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