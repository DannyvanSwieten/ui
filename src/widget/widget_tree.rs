use crate::{tree::Tree, widget::Widget};
use std::{any::Any, sync::Arc};

pub type WidgetTree = Tree<WidgetElement>;
pub struct WidgetElement {
    pub widget: Box<dyn Widget>,
    widget_state: Option<Arc<dyn Any + Send>>,
}

impl WidgetElement {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        Self {
            widget,
            widget_state: None,
        }
    }

    pub fn widget(&self) -> &dyn Widget {
        self.widget.as_ref()
    }

    pub fn widget_state<T: 'static>(&self) -> Option<&T> {
        self.widget_state
            .as_ref()
            .and_then(|s| s.downcast_ref::<T>())
    }

    pub fn state(&self) -> Option<Arc<dyn Any + Send>> {
        self.widget_state.clone()
    }

    pub fn set_state(&mut self, state: Option<Arc<dyn Any + Send>>) {
        self.widget_state = state
    }
}
