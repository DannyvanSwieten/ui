mod paint_ctx;
mod painter_tree;
mod painter_tree_builder;
pub mod render_ctx;
pub mod tree_painter;

pub use paint_ctx::PaintCtx;
pub use painter_tree::{PainterElement, PainterTree};
pub use tree_painter::TreePainter;

use crate::{animation::animation_ctx::AnimationCtx, canvas::Canvas};

pub use self::painter_tree_builder::PainterTreeBuilder;
use self::render_ctx::RenderCtx;

pub trait Painter: Send {
    fn mounted(&self, _render_ctx: &mut RenderCtx) {}
    fn animation_event(&mut self, _ctx: &mut AnimationCtx) {}
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas);
}
