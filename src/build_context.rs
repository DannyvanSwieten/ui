use crate::{ui_state::UIState, value::Var};

pub struct BuildCtx<'a> {
    pub id: usize,
    ui_state: &'a mut UIState,
}

impl<'a> BuildCtx<'a> {
    pub fn new(id: usize, ui_state: &'a mut UIState) -> Self {
        Self { id, ui_state }
    }

    pub fn bind(&mut self, name: &str) -> Option<&Var> {
        self.ui_state.bind_one(self.id, name);
        self.ui_state.get(name)
    }
}
