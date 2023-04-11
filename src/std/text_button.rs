use crate::{
    canvas::{color::Color32f, font::Font, paint::Paint, Canvas},
    constraints::BoxConstraints,
    event::MouseEvent,
    event_context::EventCtx,
    geo::{Rect, Size},
    message_context::MessageCtx,
    painter::{PaintCtx, Painter},
    ui_state::UIState,
    value::Value,
    widget::{BuildCtx, Children, LayoutCtx, SizeCtx, Widget},
};
use std::{any::Any, sync::Arc};

enum ButtonState {
    Active,
    Inactive,
    Hovered,
}

pub type ClickHandler = Option<Box<dyn Fn(&mut MessageCtx)>>;

pub struct TextButton {
    text: Value,
    click_handler: ClickHandler,
}

impl TextButton {
    pub fn new(text: impl Into<Value>) -> Self {
        Self {
            text: text.into(),
            click_handler: None,
        }
    }

    pub fn on_click<F>(mut self, click_handler: F) -> Self
    where
        F: Fn(&mut MessageCtx) + 'static,
    {
        self.click_handler = Some(Box::new(click_handler));
        self
    }
}

impl Widget for TextButton {
    fn build(&self, _build_ctx: &mut BuildCtx) -> Children {
        vec![]
    }

    fn calculate_size(
        &self,
        _children: &[usize],
        _constraints: &BoxConstraints,
        _layout_ctx: &SizeCtx,
    ) -> Option<Size> {
        let font = Font::new("Arial", 24.0);
        let font = skia_safe::Font::new(
            skia_safe::Typeface::new(font.typeface(), skia_safe::FontStyle::normal()).unwrap(),
            font.size(),
        );
        let blob = match &self.text {
            Value::Binding(_) => None,
            Value::Const(str) => skia_safe::TextBlob::new(str.to_string(), &font),
        };

        if let Some(blob) = blob {
            let bounds = blob.bounds();
            Some(Size::new(
                bounds.width() + bounds.width() * 0.1,
                bounds.height() + bounds.height() * 0.1,
            ))
        } else {
            None
        }
    }

    fn layout(&self, _ui_state: &UIState, _: &mut LayoutCtx, _: Size, _: &[usize]) {}

    fn mouse_event(
        &self,
        _ui_state: &UIState,
        event_ctx: &mut EventCtx,
        message_ctx: &mut MessageCtx,
    ) {
        match event_ctx.mouse_event() {
            MouseEvent::MouseMove(_) => event_ctx.set_state(|_| ButtonState::Hovered),
            MouseEvent::MouseDown(_) => event_ctx.set_state(|_| ButtonState::Active),
            MouseEvent::MouseUp(_) => {
                if let Some(handler) = &self.click_handler {
                    (handler)(message_ctx)
                }

                event_ctx.set_state(|_| ButtonState::Inactive)
            }
            _ => (),
        }
    }

    fn state(&self, _: &UIState) -> Option<Arc<dyn Any + Send>> {
        Some(Arc::new(ButtonState::Inactive))
    }

    fn painter(&self, ui_state: &UIState) -> Option<Box<dyn Painter>> {
        let text = self.text.var(ui_state).to_string();
        Some(Box::new(TextButtonPainter::new(text)))
    }
}

pub struct TextButtonPainter {
    active_paint: Paint,
    inactive_paint: Paint,
    hover_paint: Paint,
    text: String,
}

impl TextButtonPainter {
    pub fn new(text: String) -> Self {
        Self {
            active_paint: Paint::new(Color32f::new_grey(0.25)),
            inactive_paint: Paint::new(Color32f::new_grey(0.05)),
            hover_paint: Paint::new(Color32f::new_grey(0.15)),
            text,
        }
    }
}

impl Painter for TextButtonPainter {
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas) {
        let state = paint_ctx.state::<ButtonState>();
        if let Some(state) = state {
            match state {
                ButtonState::Active => canvas.draw_rounded_rect(
                    &Rect::new_from_size(paint_ctx.local_bounds().size()),
                    4.0,
                    4.0,
                    &self.active_paint,
                ),
                ButtonState::Inactive => canvas.draw_rounded_rect(
                    &Rect::new_from_size(paint_ctx.local_bounds().size()),
                    4.0,
                    4.0,
                    &self.inactive_paint,
                ),
                ButtonState::Hovered => canvas.draw_rounded_rect(
                    &Rect::new_from_size(paint_ctx.local_bounds().size()),
                    4.0,
                    4.0,
                    &self.hover_paint,
                ),
            }
        }

        let text_paint = Paint::new(Color32f::new_grey(1.0));
        canvas.draw_string(
            &Rect::new_from_size(paint_ctx.local_bounds().size()),
            &self.text,
            &Font::new("Arial", 24.0),
            &text_paint,
        );
    }
}
