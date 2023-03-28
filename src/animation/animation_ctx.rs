use super::animation_event::AnimationEvent;

pub struct AnimationCtx {
    event: AnimationEvent,
}

impl AnimationCtx {
    pub fn event(&self) -> &AnimationEvent {
        &self.event
    }
    pub fn cancel(&mut self) {}
    pub fn restart(&mut self) {}
}
