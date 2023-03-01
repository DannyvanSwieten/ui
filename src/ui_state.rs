use std::collections::HashMap;

use crate::value::Var;

pub struct UIState {
    values: HashMap<String, Var>,
    dependees: HashMap<String, Vec<usize>>,
    updates: Vec<usize>,
}

impl UIState {
    pub fn new() -> UIState {
        Self {
            values: HashMap::new(),
            dependees: HashMap::new(),
            updates: Vec::new(),
        }
    }

    pub fn clear_updates(&mut self) {
        self.updates.clear()
    }

    pub fn updates(&self) -> &[usize] {
        &self.updates
    }

    pub fn register(&mut self, name: &str, default_value: Var) {
        self.values.insert(name.to_string(), default_value);
    }

    pub fn set(&mut self, name: &str, value: Var) {
        self.values.insert(name.to_string(), value.clone());
        if let Some(dependees) = self.dependees.get(name) {
            self.updates.extend_from_slice(dependees);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Var> {
        self.values.get(name)
    }

    pub fn bind(&mut self, bindings: HashMap<String, Vec<usize>>) {
        for (name, bindings) in bindings {
            if !self.dependees.contains_key(&name) {
                self.dependees.insert(name.to_string(), bindings);
            } else {
                self.dependees.get_mut(&name).unwrap().extend(bindings);
            }
        }
    }

    pub fn bind_one(&mut self, id: usize, name: &str) {
        if !self.dependees.contains_key(name) {
            self.dependees.insert(name.to_string(), vec![id]);
        } else {
            self.dependees.get_mut(name).unwrap().push(id);
        }
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}
