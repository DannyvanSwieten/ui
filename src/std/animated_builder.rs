use std::{any::Any, sync::Arc, time::Duration};

use crate::{
    animation::animation_event::AnimationEvent,
    event_context::EventCtx,
    ui_state::UIState,
    widget::{BuildCtx, Children, Widget},
};

pub struct AnimatedBuilder {
    duration: Duration,
    build: Box<dyn Fn(&mut BuildCtx, f64) -> Children>,
}
#[derive(Clone, Copy)]
struct AnimatedBuilderState {
    phase: f64,
    started: bool,
}

impl AnimatedBuilder {
    pub fn new<F>(duration: Duration, build: F) -> Self
    where
        F: Fn(&mut BuildCtx, f64) -> Children + 'static,
    {
        Self {
            duration,
            build: Box::new(build),
        }
    }
}

impl Widget for AnimatedBuilder {
    fn state(&self, _: &UIState) -> Option<std::sync::Arc<dyn Any + Send>> {
        Some(Arc::new(AnimatedBuilderState {
            phase: 0.0,
            started: false,
        }))
    }

    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        let state = *build_ctx.state::<AnimatedBuilderState>().unwrap();
        if !state.started {
            build_ctx.request_widget_animation(0, self.duration)
        }
        (self.build)(build_ctx, state.phase)
    }

    fn animation_event(&self, event_context: &mut EventCtx, _ui_state: &UIState) {
        let new_state = match event_context.animation_event() {
            AnimationEvent::Start(_) => AnimatedBuilderState {
                phase: 0.0,
                started: true,
            },
            AnimationEvent::End(_) => AnimatedBuilderState {
                phase: 1.0,
                started: false,
            },
            AnimationEvent::Update(_, phase) => AnimatedBuilderState {
                phase: *phase,
                started: true,
            },
        };
        event_context.set_state(move |_old_state| new_state);
    }
}
