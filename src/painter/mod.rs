mod paint_ctx;
mod painter_tree;
mod tree_painter;

pub use paint_ctx::PaintCtx;
pub use painter_tree::{PainterElement, PainterTree};
pub use tree_painter::TreePainter;

use crate::canvas::Canvas;

pub trait Painter: Send {
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas);
}
