use std::collections::HashMap;

pub enum DataType {
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