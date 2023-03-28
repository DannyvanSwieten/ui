mod paint_ctx;
mod painter_tree;
mod painter_tree_builder;
mod tree_painter;

pub use paint_ctx::PaintCtx;
pub use painter_tree::{PainterElement, PainterTree};
pub use tree_painter::TreePainter;

use crate::canvas::Canvas;

pub use self::painter_tree_builder::PainterTreeBuilder;

pub trait Painter: Send {
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas);
}
