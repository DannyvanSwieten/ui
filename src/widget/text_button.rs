use crate::{
    build_context::BuildCtx,
    canvas::{color::Color32f, font::Font, paint::Paint, paint_ctx::PaintCtx, Canvas2D},
    constraints::BoxConstraints,
    event::MouseEvent,
    event_context::EventCtx,
    layout_ctx::LayoutCtx,
    message_context::MessageCtx,
    rect::Rect,
    size::Size2D,
    ui_state::UIState,
};

use super::{Children, Widget};

enum ButtonState {
    Active,
    Inactive,
    Hovered,
}

pub type ClickHandler = Option<Box<dyn Fn(&mut MessageCtx)>>;

pub struct TextButton {
    active_paint: Paint,
    inactive_paint: Paint,
    hover_paint: Paint,
    text: String,
    click_handler: ClickHandler,
}

impl TextButton {
    pub fn new(text: &str) -> Self {
        Self {
            active_paint: Paint::new(Color32f::new_grey(0.25)),
            inactive_paint: Paint::new(Color32f::new_grey(0.05)),
            hover_paint: Paint::new(Color32f::new_grey(0.15)),
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
        _layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        Some(Size2D::new(100.0, 50.0))
    }

    fn layout(&self, _ui_state: &UIState, _: &mut LayoutCtx, _: Size2D, _: &[usize]) {}

    fn paint(&self, paint_ctx: &PaintCtx, _: &UIState, canvas: &mut dyn Canvas2D) {
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

    fn mouse_event(
        &self,
        _ui_state: &UIState,
        event_ctx: &mut EventCtx,
        message_ctx: &mut MessageCtx,
    ) {
        match event_ctx.mouse_event() {
            MouseEvent::MouseMove(_) => event_ctx.set_state(|_| Box::new(ButtonState::Hovered)),
            MouseEvent::MouseDown(_) => event_ctx.set_state(|_| Box::new(ButtonState::Active)),
            MouseEvent::MouseUp(_) => {
                if let Some(handler) = &self.click_handler {
                    (handler)(message_ctx)
                }

                event_ctx.set_state(|_| Box::new(ButtonState::Inactive))
            }
            _ => (),
        }
    }

    fn state(&self) -> Option<Box<dyn std::any::Any>> {
        Some(Box::new(ButtonState::Inactive))
    }
}
