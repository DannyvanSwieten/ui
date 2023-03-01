use crate::event::MouseEvent;

#[derive(Clone, Copy)]
pub struct EventCtx<'a> {
    id: usize,
    drag_source: Option<usize>,
    mouse_event: Option<&'a MouseEvent>,
}

impl<'a> EventCtx<'a> {
    pub fn new(id: usize, mouse_event: Option<&'a MouseEvent>) -> Self {
        Self {
            id,
            drag_source: None,
            mouse_event,
        }
    }

    pub fn set_drag_source(&mut self) {
        self.drag_source = Some(self.id)
    }

    pub fn mouse_event(&self) -> &'a MouseEvent {
        self.mouse_event.unwrap()
    }

    pub fn drag_source(&self) -> Option<usize> {
        self.drag_source
    }
}
