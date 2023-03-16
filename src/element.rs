use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};

pub static NEXT_ELEMENT_ID: AtomicUsize = AtomicUsize::new(0);
pub fn next_element_id() -> usize {
    NEXT_ELEMENT_ID.fetch_add(1, Ordering::SeqCst) + 1
}

use crate::{
    constraints::BoxConstraints,
    geo::{Point, Rect, Size},
    painter::Painter,
    widget::{LayoutCtx, Widget},
};

pub struct PainterElement {
    painter: Box<dyn Painter>,
    state: Option<Box<dyn Any>>,
}

pub struct WidgetElement {
    widget: Box<dyn Widget>,
    state: Option<Box<dyn Any>>,
}

pub struct Element {
    widget: Box<dyn Widget>,
    widget_state: Option<Box<dyn Any>>,
    children: Vec<usize>,
    local_bounds: Rect,
    global_bounds: Rect,
}

impl Element {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        let widget_state = widget.state();
        Self {
            widget,
            children: Vec::new(),
            local_bounds: Rect::default(),
            global_bounds: Rect::default(),
            widget_state,
        }
    }

    pub fn widget(&self) -> &dyn Widget {
        self.widget.as_ref()
    }

    pub fn widget_state(&self) -> &Option<Box<dyn Any>> {
        &self.widget_state
    }

    pub fn widget_state_mut(&mut self) -> &mut Option<Box<dyn Any>> {
        &mut self.widget_state
    }

    pub fn add_child(&mut self, id: usize) {
        self.children.push(id)
    }

    pub fn add_children(&mut self, ids: Vec<usize>) {
        self.children.extend(ids)
    }

    pub fn set_local_bounds(&mut self, bounds: &Rect) {
        self.local_bounds = *bounds
    }

    pub fn local_bounds(&self) -> &Rect {
        &self.local_bounds
    }

    pub fn set_global_bounds(&mut self, bounds: &Rect) {
        self.global_bounds = *bounds
    }

    pub fn global_bounds(&self) -> &Rect {
        &self.global_bounds
    }

    pub fn children(&self) -> &[usize] {
        &self.children
    }

    pub fn children_copy(&self) -> Vec<usize> {
        self.children.clone()
    }

    pub fn set_state(&mut self, state: Box<dyn Any>) {
        self.widget_state = Some(state)
    }

    pub fn calculate_size(
        &self,
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size> {
        self.widget
            .calculate_size(&self.children, constraints, layout_ctx)
    }

    pub fn hit_test(&self, point: &Point) -> bool {
        self.global_bounds.hit_test(point)
    }
}
