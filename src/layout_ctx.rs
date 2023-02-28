use std::collections::HashMap;

use crate::{
    constraints::BoxConstraints, point::Point2D, rect::Rect, size::Size2D,
    user_interface::UserInterface,
};

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
    ) -> Option<Size2D> {
        self.user_interface
            .calculate_element_size(id, constraints, layout_ctx)
    }

    pub fn set_child_bounds(&mut self, id: usize, rect: Rect) {
        self.bounds.insert(id, rect);
    }

    pub fn set_child_position(&mut self, id: usize, position: Point2D) {
        if let Some(bounds) = self.bounds.get_mut(&id) {
            bounds.set_position(position);
        } else {
            self.bounds
                .insert(id, Rect::new(position, Size2D::new(0.0, 0.0)));
        }
    }
}
