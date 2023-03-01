use crate::{
    build_context::BuildCtx,
    canvas::{color::Color32f, font::Font, paint::Paint, paint_ctx::PaintCtx, Canvas2D},
    constraints::BoxConstraints,
    layout_ctx::LayoutCtx,
    rect::Rect,
    size::Size2D,
    value::Value,
};

use super::{Children, Widget};

pub struct Label {
    text: String,
    binding: Option<String>,
}

impl Label {
    pub fn new(default: Value) -> Self {
        match default {
            Value::Binding(binding) => Self {
                text: "".to_string(),
                binding: Some(binding),
            },
            Value::Const(c) => Self {
                text: c.to_string(),
                binding: None,
            },
        }
    }
}

impl Widget for Label {
    fn build(&mut self, build_ctx: &mut BuildCtx) -> Children {
        if let Some(binding) = &self.binding {
            if let Some(var) = build_ctx.bind(binding) {
                self.text = var.to_string()
            }
        }

        None
    }

    fn layout(&self, _layout_ctx: &mut LayoutCtx, _size: Size2D, _children: &[usize]) {}

    fn calculate_size(
        &self,
        _children: &[usize],
        _constraints: &BoxConstraints,
        _layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        Some(Size2D::new(200.0, 150.0))
    }

    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas2D) {
        let font = Font::new("Consolas", 34.0);
        let paint = Paint::new(Color32f::new_grey(1.0));
        canvas.draw_string(
            &Rect::new_from_size(paint_ctx.local_bounds().size()),
            &self.text,
            &font,
            &paint,
        )
    }
}
