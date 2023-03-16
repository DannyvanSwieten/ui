use crate::{
    canvas::{color::Color32f, font::Font, paint::Paint, paint_ctx::PaintCtx, Canvas},
    constraints::BoxConstraints,
    geo::{Rect, Size},
    ui_state::UIState,
    value::Value,
    widget::{BuildCtx, Children, LayoutCtx, Painter, Widget},
};

pub struct Label {
    binding: Option<String>,
}

impl Label {
    pub fn new(default: Value) -> Self {
        match default {
            Value::Binding(binding) => Self {
                binding: Some(binding),
            },
            Value::Const(_) => Self { binding: None },
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

    fn calculate_size(
        &self,
        _children: &[usize],
        _constraints: &BoxConstraints,
        _layout_ctx: &LayoutCtx,
    ) -> Option<Size> {
        Some(Size::new(200.0, 150.0))
    }

    fn painter(&self) -> Option<Box<dyn Painter>> {
        Some(Box::new(LabelPainter {
            binding: self.binding.clone(),
        }))
    }
}

pub struct LabelPainter {
    binding: Option<String>,
}

impl Painter for LabelPainter {
    fn paint(&self, paint_ctx: &PaintCtx, ui_state: &UIState, canvas: &mut dyn Canvas) {
        let font = Font::new("Consolas", 34.0);
        let paint = Paint::new(Color32f::new_grey(1.0));
        let text = if let Some(binding) = &self.binding {
            ui_state.get(binding)
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
