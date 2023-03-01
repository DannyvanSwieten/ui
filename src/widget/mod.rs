use crate::{
    build_context::BuildCtx,
    canvas::{paint_ctx::PaintCtx, Canvas2D},
    constraints::BoxConstraints,
    event::MouseEvent,
    layout_ctx::LayoutCtx,
    message_context::MessageCtx,
    size::Size2D,
};

pub mod center;
pub mod flex;
pub mod label;
pub mod text_button;

type Children = Option<Vec<Box<dyn Widget>>>;

pub trait Widget {
    fn build(&mut self, _build_ctx: &mut BuildCtx) -> Children {
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
    fn mouse_event(&mut self, _event: &MouseEvent, _message_ctx: &mut MessageCtx) {}
}
