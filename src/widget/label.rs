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
    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        if let Some(binding) = &self.binding {
            build_ctx.bind(binding);
        }

        vec![]
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
        let text = if let Some(binding) = &self.binding {
            paint_ctx.ui_state().get(&binding)
        } else {
            None
        };

        let text = text.unwrap();
        canvas.draw_string(
            &Rect::new_from_size(paint_ctx.local_bounds().size()),
            &text.to_string(),
            &font,
            &paint,
        )
    }
}
