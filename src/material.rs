use std::collections::HashMap;

pub type Material = HashMap<&'static str, bool>;

#[derive(Debug)]
pub struct MaterialStack {
    stack: Vec<Material>,
}

impl MaterialStack {
    pub fn new(base_material: Material) -> Self {
        Self {
            stack: vec![base_material],
        }
    }

    pub fn push(&mut self) {
        self.stack.push(Material::new());
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn set(&mut self, key: &'static str, value: bool) {
        self.stack.last_mut().unwrap().insert(key, value);
    }

    pub fn get(&self, key: &'static str) -> Option<bool> {
        for scope in self.stack.iter().rev() {
            if let Some(value) = scope.get(key) {
                return Some(*value);
            }
        }

        None
    }
}
