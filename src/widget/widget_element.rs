use super::Widget;
use crate::geo::{Point, Rect};
use std::any::Any;

pub struct WidgetElement {
    widget: Box<dyn Widget>,
    widget_state: Option<Box<dyn Any>>,
    pub local_bounds: Rect,
    pub global_bounds: Rect,
}

impl WidgetElement {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        let widget_state = widget.state();
        Self {
            widget,
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

    pub fn set_state(&mut self, state: Box<dyn Any>) {
        self.widget_state = Some(state)
    }

    pub fn hit_test(&self, point: &Point) -> bool {
        self.global_bounds.hit_test(point)
    }
}
