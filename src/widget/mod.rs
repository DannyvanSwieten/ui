use std::any::Any;

use crate::{
    build_context::BuildCtx,
    canvas::{paint_ctx::PaintCtx, Canvas2D},
    constraints::BoxConstraints,
    event_context::EventCtx,
    layout_ctx::LayoutCtx,
    message_context::MessageCtx,
    size::Size2D,
    ui_state::UIState,
};

pub mod center;
pub mod drag_source;
pub mod drop_target;
pub mod flex;
pub mod label;
pub mod text_button;

type Child = Box<dyn Fn() -> Box<dyn Widget>>;
type Children = Vec<Box<dyn Widget>>;

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
    ) -> Option<Size2D> {
        None
    }

    fn layout(
        &self,
        ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size2D,
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

pub trait Painter {
    fn paint(&self, paint_ctx: &PaintCtx, ui_state: &UIState, canvas: &mut dyn Canvas2D);
}
