use std::{any::Any, sync::Arc};

use crate::{
    ui_state::UIState,
    widget::{BuildCtx, Children, Widget},
};

pub struct AnimatedBuilder {
    build: Box<dyn Fn(&mut BuildCtx, f64) -> Children>,
}

impl AnimatedBuilder {
    pub fn new<F>(build: F) -> Self
    where
        F: Fn(&mut BuildCtx, f64) -> Children + 'static,
    {
        Self {
            build: Box::new(build),
        }
    }
}

impl Widget for AnimatedBuilder {
    fn state(&self, _: &UIState) -> Option<std::sync::Arc<dyn Any + Send>> {
        Some(Arc::new(0.0))
    }

    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        let phase = build_ctx.state::<f64>().unwrap();
        (self.build)(build_ctx, *phase)
    }
}
