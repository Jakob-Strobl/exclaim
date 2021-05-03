use std::collections::HashMap;

pub mod types;
use types::DataType;

pub mod traits;

pub struct DataContext {
    data: HashMap<String, DataType>,
}

impl DataContext {
    pub fn new() -> DataContext {
        DataContext {
            data: HashMap::new(),
        }

    }

    pub fn insert(&mut self, key: String, value: DataType) -> Option<DataType> {
        self.data.insert(key, value)
    }

    pub fn get(&mut self, key: &str) -> Option<&DataType> {
        self.data.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut DataType> {
        self.data.get_mut(key)
    }
}
