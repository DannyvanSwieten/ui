use crate::{
    canvas::{color::Color32f, font::Font, paint::Paint, Canvas},
    constraints::BoxConstraints,
    geo::{Rect, Size},
    painter::{PaintCtx, Painter},
    ui_state::UIState,
    value::Value,
    widget::{BuildCtx, Children, LayoutCtx, Widget},
};

pub struct Label {
    binding: Value,
}

impl Label {
    pub fn new(binding: Value) -> Self {
        Self { binding }
    }
}

impl Widget for Label {
    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        if let Value::Binding(binding) = &self.binding {
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

    fn painter(&self, ui_state: &UIState) -> Option<Box<dyn Painter>> {
        let text = match &self.binding {
            Value::Binding(binding) => ui_state.get(binding).unwrap(),
            Value::Const(var) => var,
        }
        .to_string();

        Some(Box::new(LabelPainter { text }))
    }
}

pub struct LabelPainter {
    text: String,
}

impl Painter for LabelPainter {
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas) {
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
