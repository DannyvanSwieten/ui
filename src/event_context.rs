use std::any::Any;

use crate::{event::MouseEvent, widget::drag_source::DragSourceData};
pub type SetState = Option<Box<dyn FnMut(&mut dyn Any)>>;

pub struct EventCtx<'a> {
    id: usize,
    drag_source: Option<DragSourceData>,
    mouse_event: Option<&'a MouseEvent>,
    set_state: SetState,
    state: &'a Option<Box<dyn Any>>,
}

impl<'a> EventCtx<'a> {
    pub fn new(
        id: usize,
        mouse_event: Option<&'a MouseEvent>,
        state: &'a Option<Box<dyn Any>>,
    ) -> Self {
        Self {
            id,
            drag_source: None,
            mouse_event,
            set_state: None,
            state,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_drag_source(&mut self, data: DragSourceData) {
        self.drag_source = Some(data)
    }

    pub fn mouse_event(&self) -> &'a MouseEvent {
        self.mouse_event.unwrap()
    }

    pub fn drag_source(&mut self) -> Option<DragSourceData> {
        self.drag_source.take()
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

    pub fn consume_state(self) -> SetState {
        self.set_state
    }

    pub fn state<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        if let Some(state) = self.state {
            state.as_ref().downcast_ref::<T>()
        } else {
            None
        }
    }
}
