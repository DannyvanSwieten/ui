use crate::{tree::ElementId, ui_state::UIState, value::Var};

pub struct BuildCtx<'a> {
    pub id: ElementId,
    ui_state: &'a mut UIState,
}

impl<'a> BuildCtx<'a> {
    pub fn new(id: ElementId, ui_state: &'a mut UIState) -> Self {
        Self { id, ui_state }
    }

    pub fn bind(&mut self, name: &str) -> Option<&Var> {
        self.ui_state.bind_one(self.id, name);
        self.ui_state.get(name)
    }

    pub fn ui_state(&'a self) -> &'a UIState {
        self.ui_state
    }
}
