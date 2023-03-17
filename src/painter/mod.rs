mod paint_ctx;
mod painter_tree;

pub use paint_ctx::PaintCtx;
pub use painter_tree::{PainterElement, PainterTree};

use crate::{canvas::Canvas, ui_state::UIState};

pub trait Painter {
    fn paint(&self, paint_ctx: &PaintCtx, ui_state: &UIState, canvas: &mut dyn Canvas);
}
