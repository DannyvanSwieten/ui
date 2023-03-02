use std::any::Any;

use crate::{
    build_context::BuildCtx,
    canvas::{paint_ctx::PaintCtx, Canvas2D},
    constraints::BoxConstraints,
    event_context::EventCtx,
    layout_ctx::LayoutCtx,
    message_context::MessageCtx,
    size::Size2D,
};

pub mod center;
pub mod drag_source;
pub mod drag_target;
pub mod flex;
pub mod label;
pub mod text_button;

type Child = Box<dyn Fn() -> Box<dyn Widget>>;
type Children = Option<Vec<Box<dyn Widget>>>;

pub trait Widget {
    fn build(&mut self, _build_ctx: &mut BuildCtx) -> Children {
        None
    }

    fn state(&self) -> Option<Box<dyn Any>> {
        None
    }

    fn calculate_size(
        &self,
        _children: &[usize],
        _constraints: &BoxConstraints,
        _layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        None
    }

    fn layout(&self, _layout_ctx: &mut LayoutCtx, _size: Size2D, _children: &[usize]) {}
    fn paint(&self, _paint_ctx: &PaintCtx, _canvas: &mut dyn Canvas2D) {}
    fn mouse_event(&mut self, _event_ctx: &mut EventCtx, _message_ctx: &mut MessageCtx) {}
    fn intercept_mouse_events(&self) -> bool {
        false
    }
}
