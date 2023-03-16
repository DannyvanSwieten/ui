mod build_ctx;
mod layout_ctx;

pub use build_ctx::BuildCtx;
pub use layout_ctx::LayoutCtx;

use crate::{
    constraints::BoxConstraints, event_context::EventCtx, geo::Size, message_context::MessageCtx,
    painter::Painter, ui_state::UIState,
};
use std::any::Any;

pub type Child = Box<dyn Fn() -> Box<dyn Widget>>;
pub type Children = Vec<Box<dyn Widget>>;

#[allow(unused_variables)]
pub trait Widget {
    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        vec![]
    }

    fn state(&self) -> Option<Box<dyn Any>> {
        None
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size> {
        None
    }

    fn layout(
        &self,
        ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
    }

    fn painter(&self) -> Option<Box<dyn Painter>> {
        None
    }

    fn mouse_event(
        &self,
        _ui_state: &UIState,
        _event_ctx: &mut EventCtx,
        _message_ctx: &mut MessageCtx,
    ) {
    }
    fn intercept_mouse_events(&self) -> bool {
        false
    }
}

impl<T> From<T> for Box<dyn Widget>
where
    T: Widget + 'static,
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
