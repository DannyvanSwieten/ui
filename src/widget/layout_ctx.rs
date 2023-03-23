use crate::{
    constraints::BoxConstraints,
    geo::{Point, Rect, Size},
    ui_state::UIState,
    value::Var,
    widget::WidgetTree,
};
use std::{any::Any, collections::HashMap, sync::Arc};

pub struct SizeCtx<'a> {
    id: usize,
    element_tree: &'a WidgetTree,
}

impl<'a> SizeCtx<'a> {
    pub fn new(id: usize, element_tree: &'a WidgetTree) -> Self {
        Self { id, element_tree }
    }

    pub fn state(&self) -> Option<Arc<dyn Any + Send>> {
        self.element_tree.state(self.id)
    }

    pub fn preferred_size(&self, id: usize, constraints: &BoxConstraints) -> Option<Size> {
        self.element_tree.calculate_element_size(id, constraints)
    }
}

pub struct LayoutCtx<'a> {
    id: usize,
    element_tree: &'a WidgetTree,
    ui_state: &'a UIState,
    bounds: HashMap<usize, Rect>,
}

impl<'a> LayoutCtx<'a> {
    pub fn new(id: usize, element_tree: &'a WidgetTree, ui_state: &'a UIState) -> Self {
        Self {
            id,
            element_tree,
            bounds: HashMap::new(),
            ui_state,
        }
    }

    pub fn binding(&self, name: &str) -> Option<&Var> {
        self.ui_state.get(name)
    }

    pub fn bounds(self) -> HashMap<usize, Rect> {
        self.bounds
    }

    pub fn preferred_size(&self, id: usize, constraints: &BoxConstraints) -> Option<Size> {
        self.element_tree.calculate_element_size(id, constraints)
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

    pub fn state(&self) -> Option<Arc<dyn Any + Send>> {
        self.element_tree.state(self.id)
    }
}
