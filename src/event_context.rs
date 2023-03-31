use std::{any::Any, sync::Arc};

use crate::{
    animation::animation_event::AnimationEvent, event::MouseEvent,
    std::drag_source::DragSourceData, tree::ElementId,
};
pub type SetState = Box<dyn Fn(&(dyn Any + Send)) -> Arc<dyn Any + Send>>;

pub struct EventCtx<'a> {
    id: ElementId,
    drag_source: Option<DragSourceData>,
    mouse_event: Option<&'a MouseEvent>,
    animation_event: Option<&'a AnimationEvent>,
    set_state: Option<SetState>,
    state: Option<&'a (dyn Any + Send)>,
}

impl<'a> EventCtx<'a> {
    pub fn new_mouse_event(
        id: ElementId,
        mouse_event: Option<&'a MouseEvent>,
        state: Option<&'a (dyn Any + Send)>,
    ) -> Self {
        Self {
            id,
            drag_source: None,
            mouse_event,
            animation_event: None,
            set_state: None,
            state,
        }
    }

    pub fn new_animation_event(
        id: ElementId,
        animation_event: Option<&'a AnimationEvent>,
        state: Option<&'a (dyn Any + Send)>,
    ) -> Self {
        Self {
            id,
            drag_source: None,
            mouse_event: None,
            animation_event,
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

    pub fn animation_event(&self) -> &'a AnimationEvent {
        self.animation_event.unwrap()
    }

    pub fn drag_source(&mut self) -> Option<DragSourceData> {
        self.drag_source.take()
    }

    pub fn set_state<T>(&mut self, modify: impl Fn(&T) -> T + Send + 'static)
    where
        T: Any + Send + 'static,
    {
        self.set_state = Some(Box::new(move |any| {
            Arc::new(modify(any.downcast_ref::<T>().unwrap()))
        }));
    }

    pub fn consume_state(self) -> Option<SetState> {
        self.set_state
    }

    pub fn state<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        if let Some(state) = self.state {
            state.downcast_ref::<T>()
        } else {
            None
        }
    }
}
