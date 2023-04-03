use std::{any::Any, sync::Arc, time::Duration};

use crate::{
    animation::{animation_request::AnimationRequest, AnimationId},
    tree::ElementId,
    ui_state::UIState,
    value::Var,
};

pub struct BuildCtx<'a> {
    pub id: ElementId,
    ui_state: &'a UIState,
    animation_requests: Vec<AnimationRequest>,
    widget_state: Option<Arc<dyn Any + Send>>,
}

impl<'a> BuildCtx<'a> {
    pub fn new(
        id: ElementId,
        widget_state: Option<Arc<dyn Any + Send>>,
        ui_state: &'a UIState,
    ) -> Self {
        Self {
            id,
            widget_state,
            ui_state,
            animation_requests: Vec::new(),
        }
    }

    pub fn state<T: Any>(&self) -> Option<&T> {
        self.widget_state
            .as_ref()
            .map(|arc| arc.downcast_ref::<T>().unwrap())
    }

    pub fn bind(&mut self, name: &str) -> Option<&Var> {
        // self.ui_state.bind_one(self.id, name);
        self.ui_state.get(name)
    }

    pub fn ui_state(&'a self) -> &'a UIState {
        self.ui_state
    }

    pub fn request_painter_animation(&mut self, id: AnimationId, duration: Duration) {
        self.animation_requests
            .push(AnimationRequest::Painter(id, duration));
    }

    pub fn request_widget_animation(&mut self, id: AnimationId, duration: Duration) {
        self.animation_requests
            .push(AnimationRequest::Widget(id, duration));
    }

    pub fn animation_requests(&self) -> Vec<AnimationRequest> {
        self.animation_requests.clone()
    }
}
