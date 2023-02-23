use std::collections::HashMap;

use crate::{constraints::BoxConstraints, rect::Rect, user_interface::UserInterface};

pub struct LayoutCtx<'a> {
    user_interface: &'a UserInterface,
    bounds: HashMap<usize, Rect>,
}

impl<'a> LayoutCtx<'a> {
    pub fn new(user_interface: &'a UserInterface) -> Self {
        Self {
            user_interface,
            bounds: HashMap::new(),
        }
    }

    pub fn bounds(self) -> HashMap<usize, Rect> {
        self.bounds
    }

    pub fn preferred_size(
        &self,
        id: usize,
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<(f32, f32)> {
        self.user_interface
            .calculate_element_size(id, constraints, layout_ctx)
    }

    pub fn set_child_rect(&mut self, id: usize, rect: Rect) {
        self.bounds.insert(id, rect);
    }
}
