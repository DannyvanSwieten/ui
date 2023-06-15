use std::{any::Any, sync::Arc, time::Duration};

use crate::{
    animation::{
        animation_event::AnimationEvent, animation_request::AnimationRequest, AnimationId,
    },
    app::event::MouseEvent,
    tree::ElementId,
    widget::Widget, user_interface::{ui_state, value::Var},
};
pub type SetState = Box<dyn Fn(&(dyn Any + Send)) -> Arc<dyn Any + Send>>;

pub struct EventCtx<'a> {
    id: ElementId,
    ui_state: &'a ui_state::UIState,
    is_mouse_over: bool,
    pub drag_data: Option<Box<dyn Any>>,
    pub drag_widget: Option<Box<dyn Widget>>,
    mouse_event: Option<&'a MouseEvent>,
    binding_event: Option<&'a str>,
    animation_event: Option<&'a AnimationEvent>,
    animation_requests: Vec<AnimationRequest>,
    set_state: Option<SetState>,
    state: Option<&'a (dyn Any + Send)>,
}

pub struct Consumed {
    pub animation_requests: Vec<AnimationRequest>,
    pub set_state: Option<SetState>,
    pub drag_data: Option<Box<dyn Any>>,
    pub drag_widget: Option<Box<dyn Widget>>,
}

impl<'a> EventCtx<'a> {
    pub fn new_mouse_event(
        id: ElementId,
        is_mouse_over: bool,
        mouse_event: Option<&'a MouseEvent>,
        ui_state: &'a ui_state::UIState,
        state: Option<&'a (dyn Any + Send)>,
    ) -> Self {
        Self {
            id,
            ui_state,
            is_mouse_over,
            drag_data: None,
            drag_widget: None,
            mouse_event,
            binding_event: None,
            animation_event: None,
            set_state: None,
            state,
            animation_requests: Vec::new(),
        }
    }

    pub fn new_animation_event(
        id: ElementId,
        animation_event: Option<&'a AnimationEvent>,
        ui_state: &'a ui_state::UIState,
        state: Option<&'a (dyn Any + Send)>,
    ) -> Self {
        Self {
            id,
            ui_state,
            is_mouse_over: false,
            drag_data: None,
            drag_widget: None,
            mouse_event: None,
            binding_event: None,
            animation_event,
            set_state: None,
            state,
            animation_requests: Vec::new(),
        }
    }

    pub fn new_binding_event(
        id: ElementId,
        binding_event: Option<&'a str>,
        ui_state: &'a ui_state::UIState,
        state: Option<&'a (dyn Any + Send)>,
    ) -> Self {
        Self {
            id,
            ui_state,
            is_mouse_over: false,
            drag_data: None,
            drag_widget: None,
            mouse_event: None,
            binding_event,
            animation_event: None,
            set_state: None,
            state,
            animation_requests: Vec::new(),
        }
    }

    pub fn consume(self) -> Consumed {
        Consumed {
            animation_requests: self.animation_requests,
            set_state: self.set_state,
            drag_data: self.drag_data,
            drag_widget: self.drag_widget,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_drag_source<T: 'static>(&mut self, widget: Box<dyn Widget>, data: T) {
        self.drag_data = Some(Box::new(data));
        self.drag_widget = Some(widget);
    }

    pub fn mouse_event(&self) -> &'a MouseEvent {
        self.mouse_event.unwrap()
    }

    pub fn request_widget_animation(&mut self, animation_id: AnimationId, duration: Duration) {
        self.animation_requests
            .push(AnimationRequest::Widget(animation_id, duration));
    }

    pub fn request_painter_animation(&mut self, animation_id: AnimationId, duration: Duration) {
        self.animation_requests
            .push(AnimationRequest::Painter(animation_id, duration));
    }

    pub fn animation_event(&self) -> &'a AnimationEvent {
        self.animation_event.unwrap()
    }

    pub fn drag_data<T: 'static>(&mut self) -> Option<&T> {
        self.drag_data
            .as_ref()
            .map(|any| any.downcast_ref::<T>().unwrap())
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

    pub fn is_mouse_over(&self) -> bool {
        self.is_mouse_over
    }

    pub fn binding(&self) -> Option<&Var> {
        self.ui_state.get(self.binding_event.unwrap())
    }
}
