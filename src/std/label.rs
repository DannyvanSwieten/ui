use std::{any::Any, sync::Arc};

use crate::{
    canvas::{color::Color32f, font::Font, paint::Paint, Canvas},
    constraints::BoxConstraints,
    geo::{Rect, Size},
    painter::{PaintCtx, Painter},
    ui_state::UIState,
    value::Value,
    widget::{BuildCtx, ChangeResponse, Children, LayoutCtx, Widget},
};

pub struct Label {
    text: Value,
}

impl Label {
    pub fn new(text: impl Into<Value>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for Label {
    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        if let Value::Binding(binding) = &self.text {
            build_ctx.bind(binding);
        }

        vec![]
    }

    fn binding_changed(&self, _: &str) -> Option<ChangeResponse> {
        Some(ChangeResponse::Layout)
    }

    fn calculate_size(
        &self,
        _children: &[usize],
        _constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size> {
        let size = if let Some(state) = layout_ctx.state() {
            if let Some(blob) = state.downcast_ref::<skia_safe::TextBlob>() {
                let bounds = blob.bounds();
                Some(Size::new(bounds.width(), bounds.height()))
            } else {
                None
            }
        } else {
            None
        };

        size
    }

    fn state(&self, ui_state: &UIState) -> Option<Arc<dyn Any + Send>> {
        let text = match &self.text {
            Value::Binding(name) => ui_state.get(name).map(|text| text.to_string()),
            Value::Const(text) => Some(text.to_string()),
        };

        let blob = if let Some(text) = text {
            let font = Font::new("Arial", 32.0);
            let font = skia_safe::Font::new(
                skia_safe::Typeface::new(font.typeface(), skia_safe::FontStyle::normal()).unwrap(),
                font.size(),
            );
            skia_safe::TextBlob::new(text, &font)
        } else {
            None
        };

        Some(Arc::new(blob))
    }

    fn painter(&self, ui_state: &UIState) -> Option<Box<dyn Painter>> {
        let text = self.text.var(ui_state).to_string();

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
