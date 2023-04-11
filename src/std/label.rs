use std::{any::Any, sync::Arc};

use crate::{
    canvas::{color::Color32f, font::Font, paint::Paint, Canvas},
    constraints::BoxConstraints,
    geo::{Rect, Size},
    painter::{PaintCtx, Painter},
    ui_state::UIState,
    value::Value,
    widget::{BuildCtx, ChangeResponse, Children, SizeCtx, Widget},
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
        Some(ChangeResponse::Build)
    }

    fn calculate_size(
        &self,
        _children: &[usize],
        _constraints: &BoxConstraints,
        size_ctx: &SizeCtx,
    ) -> Option<Size> {
        let size = if let Some(state) =
            size_ctx.state::<Option<(String, Option<skia_safe::TextBlob>)>>()
        {
            match state {
                Some((_, blob)) => {
                    let bounds = blob.as_ref().unwrap().bounds();
                    Some(Size::new(bounds.width(), bounds.height()))
                }
                _ => None,
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
            Some((text.clone(), skia_safe::TextBlob::new(text, &font)))
        } else {
            None
        };

        Some(Arc::new(blob))
    }

    fn painter(&self, _: &UIState) -> Option<Box<dyn Painter>> {
        Some(Box::new(LabelPainter {}))
    }
}

pub struct LabelPainter {}

impl Painter for LabelPainter {
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas) {
        let font = Font::new("Arial", 32.0);
        let paint = Paint::new(Color32f::new_grey(1.0));
        let state = paint_ctx.state::<Option<(String, Option<skia_safe::TextBlob>)>>();

        if let Some(state) = state {
            match state {
                Some((text, _)) => canvas.draw_string(
                    &Rect::new_from_size(paint_ctx.local_bounds().size()),
                    text,
                    &font,
                    &paint,
                ),
                None => todo!(),
            }
        }
    }
}
