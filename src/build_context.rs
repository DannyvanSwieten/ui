use std::collections::HashMap;

use crate::{ui_state::UIState, value::Var};

pub struct BuildCtx<'a> {
    id: usize,
    bindings: HashMap<String, Vec<usize>>,
    ui_state: &'a UIState,
}

impl<'a> BuildCtx<'a> {
    pub fn new(id: usize, ui_state: &'a UIState) -> Self {
        Self {
            id,
            bindings: HashMap::new(),
            ui_state,
        }
    }

    pub fn bind(&mut self, name: &str) -> Option<&Var> {
        if !self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), Vec::new());
        }
        self.bindings.get_mut(name).unwrap().push(self.id);
        self.ui_state.get(name)
    }

    pub fn bindings(self) -> HashMap<String, Vec<usize>> {
        self.bindings
    }
}
