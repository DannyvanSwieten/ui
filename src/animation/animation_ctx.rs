use super::animation_event::AnimationEvent;

pub struct AnimationCtx<'a> {
    event: &'a AnimationEvent,
    restart_requested: bool,
    cancel_requested: bool,
}

impl<'a> AnimationCtx<'a> {
    pub fn new(event: &'a AnimationEvent) -> Self {
        Self {
            event,
            restart_requested: false,
            cancel_requested: false,
        }
    }
    pub fn event(&self) -> &AnimationEvent {
        self.event
    }
    pub fn cancel(&mut self) {
        self.cancel_requested = true;
    }
    pub fn restart(&mut self) {
        self.restart_requested = true;
    }
}
