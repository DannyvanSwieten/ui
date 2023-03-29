use crate::tree::ElementId;

use super::{animation_event::AnimationEvent, animation_request::AnimationRequest};

pub struct AnimationCtx {
    id: ElementId,
    event: AnimationEvent,
    request: Option<AnimationRequest>,
}

impl AnimationCtx {
    pub fn new(id: ElementId, event: AnimationEvent) -> Self {
        Self {
            id,
            event,
            request: None,
        }
    }
    pub fn event(&self) -> &AnimationEvent {
        &self.event
    }
    pub fn cancel(&mut self) {}
    pub fn restart(&mut self) {}
}
