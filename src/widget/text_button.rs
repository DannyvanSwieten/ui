use crate::{
    build_context::BuildCtx,
    canvas::{color::Color32f, font::Font, paint::Paint, paint_ctx::PaintCtx, Canvas2D},
    constraints::BoxConstraints,
    event::MouseEvent,
    layout_ctx::LayoutCtx,
    message_context::MessageCtx,
    rect::Rect,
    size::Size2D,
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
    state: ButtonState,
    text: String,
    click_handler: ClickHandler,
}

impl TextButton {
    pub fn new(text: &str) -> Self {
        Self {
            state: ButtonState::Inactive,
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
    fn build(&mut self, _build_ctx: &mut BuildCtx) -> Children {
        None
    }

    fn calculate_size(
        &self,
        _children: &[usize],
        _constraints: &BoxConstraints,
        _layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        Some(Size2D::new(100.0, 50.0))
    }

    fn layout(&self, _: &mut LayoutCtx, _: Size2D, _: &[usize]) {}

    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas2D) {
        match self.state {
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

        let text_paint = Paint::new(Color32f::new_grey(1.0));
        canvas.draw_string(
            &Rect::new_from_size(paint_ctx.local_bounds().size()),
            &self.text,
            &Font::new("Arial", 24.0),
            &text_paint,
        );
    }

    fn mouse_event(&mut self, event: &MouseEvent, message_ctx: &mut MessageCtx) {
        match event {
            MouseEvent::MouseMove(_) => self.state = ButtonState::Hovered,
            MouseEvent::MouseEnter(_) => (),
            MouseEvent::MouseLeave(_) => (),
            MouseEvent::MouseUp(_) => {
                self.state = ButtonState::Inactive;
                if let Some(handler) = &self.click_handler {
                    (handler)(message_ctx)
                }
            }
            MouseEvent::MouseDown(_) => self.state = ButtonState::Active,
            MouseEvent::MouseDrag(_) => (),
            MouseEvent::MouseDragStart(_) => (),
            MouseEvent::MouseDragEnd(_) => (),
        }
    }
}
