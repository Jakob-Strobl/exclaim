use std::collections::HashMap;

use super::traits::Renderable;

pub enum DataType {
    // Reserved - value not initialized
    Any, 

    // Scalar
    String(String),
    Int(isize),
    Uint(usize),
    Float(f64),

    // Compound Types
    Tuple(Box<[DataType]>),
    Object(HashMap<String, DataType>),
    Array(Vec<DataType>),
}

impl DataType {
    pub fn is_scalar(&self) -> bool {
        match self {
            DataType::String(_) => true,
            DataType::Int(_) => true,
            DataType::Uint(_) => true,
            DataType::Float(_) => true,
            _ => false,
        }
    }
}

impl Renderable for DataType {
    fn render(&self) -> String {
        match self {
            DataType::Any => panic!("DataType::Any can not be rendered!"),
            DataType::String(s) => s.to_string(),
            DataType::Int(num) => num.to_string(),
            DataType::Uint(num) => num.to_string(),
            DataType::Float(num) => num.to_string(),
            DataType::Tuple(_) => panic!("DataType::Tuple unimplemented!"),
            DataType::Object(_) => panic!("DataType::Object unimplemented!"),
            DataType::Array(_) => panic!("DataType::Array unimplemented!"),
        }
    }
}