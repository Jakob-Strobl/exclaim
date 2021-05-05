use super::ScopeContext;
use super::DataContext;
use super::Renderable;
use super::Data;

pub struct RuntimeContext {
    output: String,
    scope_ctx: ScopeContext,
    // Global context is data that is not found inside the template 
    global_ctx: DataContext,
}

impl RuntimeContext {
    pub fn new() -> RuntimeContext {
        RuntimeContext {
            output: String::new(),
            scope_ctx: ScopeContext::new(),
            global_ctx: DataContext::new(),
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

    pub fn render_from_context(&mut self, key: &str) {
        // Check scope context first
        if let Some(data) = self.scope_ctx.get(key) {
            self.output.push_str(&data.render());
        } else {
            // Check global context
            if let Some(data) = self.global_ctx.get(key) {
                self.output.push_str(&data.render());
            } else {
                panic!("Render From Context: Error could not find data with the key '{}'", key);
            }
        }
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