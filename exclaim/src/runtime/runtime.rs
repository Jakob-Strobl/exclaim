use super::ScopeContext;
use super::DataContext;
use super::Renderable;
use super::Data;

pub struct RuntimeContext {
    output: String,
    scope_ctx: ScopeContext,
    // Global context is data that is not found inside the template 
    // TODO Instead of using a DataContext, create a more user friendly API for global data. for now, this is fine
    global_ctx: DataContext,
}

impl RuntimeContext {
    pub fn new(global: Option<DataContext>) -> RuntimeContext {
        RuntimeContext {
            output: String::new(),
            scope_ctx: ScopeContext::new(),
            global_ctx: global.unwrap_or(DataContext::new()),
        }
    }

    pub fn open_scope(&mut self) {
        self.scope_ctx.open_scope();
    }

    pub fn close_scope(&mut self) {
        self.scope_ctx.close_scope();
    }

    pub fn insert(&mut self, key: String, value: Data) {
        self.scope_ctx.insert(key, value)
    }

    pub fn render(&mut self, item: &dyn Renderable) {
        self.output.push_str(&item.render())
    }

    pub fn get(&self, key: &str) -> &Data {
        if let Some(data) = self.scope_ctx.get(key) {
            data
        } else {
            // Check global context
            if let Some(data) = self.global_ctx.get(key) {
                data
            } else {
                panic!("get: Error could not find data with the key '{}'", key);
            }
        }
    }

    pub fn output(self) -> String {
        self.output
    }
}