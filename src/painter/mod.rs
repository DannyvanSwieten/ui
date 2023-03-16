mod paint_ctx;

pub use paint_ctx::PaintCtx;

use crate::{canvas::Canvas, ui_state::UIState};

pub trait Painter {
    fn paint(&self, paint_ctx: &PaintCtx, ui_state: &UIState, canvas: &mut dyn Canvas);
}
