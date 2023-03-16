use std::collections::HashMap;

use crate::{
    constraints::BoxConstraints,
    element_tree::WidgetTree,
    geo::{Point, Rect, Size},
};

pub struct LayoutCtx<'a> {
    element_tree: &'a WidgetTree,
    bounds: HashMap<usize, Rect>,
}

impl<'a> LayoutCtx<'a> {
    pub fn new(element_tree: &'a WidgetTree) -> Self {
        Self {
            element_tree,
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
    ) -> Option<Size> {
        self.element_tree
            .calculate_element_size(id, constraints, layout_ctx)
    }

    pub fn set_child_bounds(&mut self, id: usize, rect: Rect) {
        self.bounds.insert(id, rect);
    }

    pub fn set_child_position(&mut self, id: usize, position: Point) {
        if let Some(bounds) = self.bounds.get_mut(&id) {
            bounds.set_position(position);
        } else {
            self.bounds
                .insert(id, Rect::new(position, Size::new(0.0, 0.0)));
        }
    }
}
