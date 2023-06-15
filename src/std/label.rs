use std::{any::Any, sync::Arc};

use crate::{
    canvas::{color::Color32f, font::Font, paint::Paint, text::Text, Canvas},
    event_context::EventCtx,
    geo::{Rect, Size},
    painter::{PaintCtx, Painter},
    user_interface::{ui_state::UIState, value::Value},
    widget::{constraints::BoxConstraints, BuildCtx, ChangeResponse, Children, SizeCtx, Widget},
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

    fn binding_changed(&self, event_context: &mut EventCtx) {
        if let Value::Binding(_) = &self.text {
            let text = event_context.binding().map(|text| text.to_string());

            if let Some(text) = text {
                event_context
                    .set_state(move |_old_state| Text::new(&text, Font::new("Arial", 24.0)));
            }
        }
    }

    fn calculate_size(
        &self,
        _children: &[usize],
        _constraints: &BoxConstraints,
        size_ctx: &SizeCtx,
    ) -> Option<Size> {
        size_ctx.state::<Text>().map(|state| state.bounds().size())
    }

    fn state(&self, ui_state: &UIState) -> Option<Arc<dyn Any + Send>> {
        let text = match &self.text {
            Value::Binding(name) => ui_state.get(name).map(|text| text.to_string()),
            Value::Const(text) => Some(text.to_string()),
        };

        if let Some(text) = text {
            let font = Font::new("Arial", 24.0);
            Some(Arc::new(Text::new(&text, font)))
        } else {
            None
        }
    }

    fn painter(&self, _: &UIState) -> Option<Box<dyn Painter>> {
        Some(Box::new(LabelPainter {}))
    }
}

pub struct LabelPainter {}

impl Painter for LabelPainter {
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas) {
        let paint = Paint::new(Color32f::new_grey(1.0));

        if let Some(state) = paint_ctx.state::<Text>() {
            canvas.draw_text(
                state,
                &Rect::new_from_size(paint_ctx.local_bounds().size()),
                &paint,
            )
        }
    }
}
