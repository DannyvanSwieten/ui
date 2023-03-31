use super::animation_event::AnimationEvent;

pub struct AnimationCtx<'a> {
    event: &'a AnimationEvent,
}

impl<'a> AnimationCtx<'a> {
    pub fn new(event: &'a AnimationEvent) -> Self {
        Self { event }
    }
    pub fn event(&self) -> &AnimationEvent {
        self.event
    }
    pub fn cancel(&mut self) {}
    pub fn restart(&mut self) {}
}
