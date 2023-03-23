use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use crate::value::Var;

pub struct UIState {
    values: HashMap<String, Var>,
    dependees: HashMap<String, Vec<usize>>,
    updates: HashMap<String, usize>,
}

impl UIState {
    pub fn new() -> UIState {
        Self {
            values: HashMap::new(),
            dependees: HashMap::new(),
            updates: HashMap::new(),
        }
    }

    pub fn clear_updates(&mut self) {
        self.updates.clear()
    }

    pub fn updates(&self) -> &HashMap<String, usize> {
        &self.updates
    }

    /// register a piece of state
    pub fn register(&mut self, name: &str, default_value: impl Into<Var>) {
        self.values.insert(name.to_string(), default_value.into());
    }

    pub fn set(&mut self, name: &str, value: impl Into<Var>) {
        self.values.insert(name.to_string(), value.into());
        if let Some(dependees) = self.dependees.get(name) {
            for dependee in dependees {
                self.updates.insert(name.to_string(), *dependee);
            }
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

impl Index<&str> for UIState {
    type Output = Var;

    fn index(&self, index: &str) -> &Self::Output {
        self.values.get(index).unwrap()
    }
}

impl IndexMut<&str> for UIState {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.values.get_mut(index).unwrap()
    }
}
