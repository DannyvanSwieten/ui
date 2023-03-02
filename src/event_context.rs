use std::any::Any;

use crate::event::MouseEvent;
pub type SetState = Option<Box<dyn FnMut(&mut dyn Any)>>;

pub struct EventCtx<'a> {
    id: usize,
    drag_source: Option<usize>,
    mouse_event: Option<&'a MouseEvent>,
    set_state: SetState,
}

impl<'a> EventCtx<'a> {
    pub fn new(id: usize, mouse_event: Option<&'a MouseEvent>) -> Self {
        Self {
            id,
            drag_source: None,
            mouse_event,
            set_state: None,
        }
    }

    pub fn set_drag_source(&mut self) {
        self.drag_source = Some(self.id)
    }

    pub fn set_drag_source_id(&mut self, id: usize) {}

    pub fn mouse_event(&self) -> &'a MouseEvent {
        self.mouse_event.unwrap()
    }

    pub fn drag_source(&self) -> Option<usize> {
        self.drag_source
    }

    pub fn set_state<F>(&mut self, s: F)
    where
        F: FnMut(&mut dyn Any) + 'static,
    {
        self.set_state = Some(Box::new(s))
    }

    pub fn state_mut(&mut self) -> &mut SetState {
        &mut self.set_state
    }
}
