use crate::tree::ElementId;

use super::animation_event::AnimationEvent;

pub struct AnimationCtx {
    id: ElementId,
    event: AnimationEvent,
}

impl AnimationCtx {
    pub fn new(id: ElementId, event: AnimationEvent) -> Self {
        Self { id, event }
    }
    pub fn event(&self) -> &AnimationEvent {
        &self.event
    }
    pub fn cancel(&mut self) {}
    pub fn restart(&mut self) {}
}
