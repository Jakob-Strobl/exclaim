use crate::data::DataContext;
use crate::data::Data;

pub struct ScopeContext {
    scopes: Vec<DataContext>
}

impl ScopeContext {
    pub fn new() -> ScopeContext {
        ScopeContext {
            scopes: vec![DataContext::new()],
        }
    }

    pub fn scope_level(&self) -> usize {
        self.scopes.len()
    }

    pub fn open_scope(&mut self) {
        self.scopes.push(DataContext::new());
    }

    pub fn close_scope(&mut self) {
        match self.scopes.pop() {
            Some(_) => (),
            None => panic!("ScopeContext Error: Tried to close scope when there is no scope!")
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Data> {
        for idx in (0..self.scopes.len()).rev() {
            if let Some(data) = self.scopes.get(idx).unwrap().get(key) {
                return Some(data)
            }
        }

        None
    }

    // Insert key-value pair in current scope
    pub fn insert(&mut self, key: String, value: Data) {
        let current_idx = self.scopes.len() - 1;
        self.scopes.get_mut(current_idx).unwrap().insert(key, value);
    }
}