use std::{any::Any, sync::Arc, time::Duration};

use crate::{
    animation::{
        animation_event::AnimationEvent, animation_request::AnimationRequest, AnimationId,
    },
    app::event::MouseEvent,
    user_interface::{ui_state::UIState, value::Var},
    widget::{ui_message::UIMessage, Widget},
};

pub enum UIEvent<'a> {
    Mouse(&'a MouseEvent),
    Animation(&'a AnimationEvent),
    Binding(&'a str),
    Internal(&'a UIMessage),
}

pub type SetState = Box<dyn Fn(&(dyn Any + Send)) -> Arc<dyn Any + Send>>;

pub struct EventCtx<'a> {
    event: UIEvent<'a>,
    ui_state: &'a UIState,
    pub drag_data: Option<Box<dyn Any>>,
    pub drag_widget: Option<Box<dyn Widget>>,
    animation_requests: Vec<AnimationRequest>,
}

impl<'a> EventCtx<'a> {
    pub fn new(event: UIEvent<'a>, ui_state: &'a UIState) -> Self {
        Self {
            event,
            ui_state,
            drag_data: None,
            drag_widget: None,
            animation_requests: Vec::new(),
        }
    }

    pub fn set_drag_source<T: 'static>(&mut self, widget: Box<dyn Widget>, data: T) {
        self.drag_data = Some(Box::new(data));
        self.drag_widget = Some(widget);
    }

    pub fn mouse_event(&self) -> &'a MouseEvent {
        match self.event {
            UIEvent::Mouse(event) => event,
            _ => panic!("Event is not a mouse event"),
        }
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
        match self.event {
            UIEvent::Animation(event) => event,
            _ => panic!("Event is not an animation event"),
        }
    }

    pub fn drag_data<T: 'static>(&mut self) -> Option<&T> {
        self.drag_data
            .as_ref()
            .map(|any| any.downcast_ref::<T>().unwrap())
    }

    pub fn binding(&self) -> Option<&Var> {
        match self.event {
            UIEvent::Binding(binding) => self.ui_state.get(binding),
            _ => panic!("Event is not a binding event"),
        }
    }

    pub fn ui_message(&self) -> &UIMessage {
        match self.event {
            UIEvent::Internal(message) => message,
            _ => panic!("Event is not an internal event"),
        }
    }
}
