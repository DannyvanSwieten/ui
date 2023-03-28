use crate::{geo::Rect, painter::Painter, tree::Tree};
use std::{any::Any, sync::Arc};

pub type PainterTree = Tree<PainterElement>;

pub struct PainterElement {
    pub painter: Option<Box<dyn Painter>>,
    painter_state: Option<Arc<dyn Any + Send>>,
    pub local_bounds: Rect,
    pub global_bounds: Rect,
}
unsafe impl Send for PainterElement {}
impl PainterElement {
    pub fn new(
        painter: Option<Box<dyn Painter>>,
        painter_state: Option<Arc<dyn Any + Send>>,
    ) -> Self {
        Self {
            painter,
            painter_state,
            local_bounds: Rect::default(),
            global_bounds: Rect::default(),
        }
    }

    pub fn with_bounds(mut self, global_bounds: &Rect, local_bounds: &Rect) -> Self {
        self.global_bounds = *global_bounds;
        self.local_bounds = *local_bounds;
        self
    }

    pub fn painter_state(&self) -> Option<&(dyn Any + Send)> {
        self.painter_state.as_deref()
    }

    pub fn painter(&self) -> &Option<Box<dyn Painter>> {
        &self.painter
    }

    pub fn set_state(&mut self, state: Option<Arc<dyn Any + Send>>) {
        self.painter_state = state
    }
}
