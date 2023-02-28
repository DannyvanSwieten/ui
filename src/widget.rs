use crate::{
    build_context::BuildCtx,
    canvas::{color::Color32f, font::Font, paint::Paint, paint_ctx::PaintCtx, Canvas2D},
    constraints::BoxConstraints,
    event::MouseEvent,
    layout_ctx::LayoutCtx,
    point::Point2D,
    rect::Rect,
    size::Size2D,
    ui_state::UIState,
    value::Value,
};

type Children = Option<Vec<Box<dyn Widget>>>;

pub trait Widget {
    fn build(&mut self, build_ctx: &mut BuildCtx) -> Children {
        None
    }
    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        None
    }

    fn layout(&self, layout_ctx: &mut LayoutCtx, size: Size2D, children: &[usize]) {}
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas2D) {}
    fn mouse_event(&mut self, event: &MouseEvent) {}
}

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

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        Some(Size2D::new(200.0, 150.0))
    }

    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas2D) {
        let font = Font::new("Consolas", 34.0);
        let paint = Paint::new(Color32f::new_grey(1.0));
        canvas.draw_string(paint_ctx.local_bounds(), &self.text, &font, &paint)
    }
}

pub struct Center {
    child: Box<dyn Fn() -> Box<dyn Widget>>,
}

impl Center {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn() -> Box<dyn Widget> + 'static,
    {
        Self {
            child: Box::new(child),
        }
    }
}

enum ButtonState {
    Active,
    Inactive,
    Hovered,
}

pub struct TextButton {
    active_paint: Paint,
    inactive_paint: Paint,
    hover_paint: Paint,
    state: ButtonState,
    text: String,
}

impl TextButton {
    pub fn new(text: &str) -> Self {
        Self {
            state: ButtonState::Inactive,
            active_paint: Paint::new(Color32f::new_grey(0.25)),
            inactive_paint: Paint::new(Color32f::new_grey(0.05)),
            hover_paint: Paint::new(Color32f::new_grey(0.15)),
            text: text.into(),
        }
    }
}

impl Widget for TextButton {
    fn build(&mut self, build_ctx: &mut BuildCtx) -> Children {
        None
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        Some(Size2D::new(100.0, 100.0))
    }

    fn layout(&self, _: &mut LayoutCtx, _: Size2D, _: &[usize]) {}

    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas2D) {
        match self.state {
            ButtonState::Active => {
                canvas.draw_rounded_rect(paint_ctx.local_bounds(), 4.0, 4.0, &self.active_paint)
            }
            ButtonState::Inactive => {
                canvas.draw_rounded_rect(paint_ctx.local_bounds(), 4.0, 4.0, &self.inactive_paint)
            }
            ButtonState::Hovered => {
                canvas.draw_rounded_rect(paint_ctx.local_bounds(), 4.0, 4.0, &self.hover_paint)
            }
        }

        let text_paint = Paint::new(Color32f::new_grey(1.0));
        canvas.draw_string(
            paint_ctx.local_bounds(),
            &self.text,
            &Font::new("Arial", 24.0),
            &text_paint,
        );
    }

    fn mouse_event(&mut self, event: &MouseEvent) {
        match event {
            MouseEvent::MouseMove(_) => self.state = ButtonState::Hovered,
            MouseEvent::MouseEnter(_) => (),
            MouseEvent::MouseLeave(_) => (),
            MouseEvent::MouseUp(_) => self.state = ButtonState::Inactive,
            MouseEvent::MouseDown(_) => self.state = ButtonState::Active,
            MouseEvent::MouseDrag(_) => (),
        }
    }
}

impl Widget for Center {
    fn build(&mut self, build_ctx: &mut BuildCtx) -> Children {
        Some(vec![(*self.child)()])
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        _: &LayoutCtx,
    ) -> Option<Size2D> {
        // Something, Somewhere, went terribly wrong
        assert_eq!(1, children.len());

        // Return all the space that is given to this widget.
        Some(Size2D::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        ))
    }

    fn layout(&self, layout_ctx: &mut LayoutCtx, size: Size2D, children: &[usize]) {
        // Something, Somewhere, went terribly wrong
        assert_eq!(1, children.len());

        let (center_x, center_y) = (size.width / 2.0, size.height / 2.0);
        let child_size = if let Some(child_size) = layout_ctx.preferred_size(
            children[0],
            &BoxConstraints::new_with_max(size.width, size.height),
            layout_ctx,
        ) {
            child_size
        } else {
            size
        };

        let position = Point2D {
            x: center_x - child_size.width / 2.0,
            y: center_y - child_size.height / 2.0,
        };

        layout_ctx.set_child_bounds(children[0], Rect::new(position, child_size))
    }
}

pub struct Row {
    children: Box<dyn Fn() -> Children>,
}

impl Row {
    pub fn new<F>(children: F) -> Self
    where
        F: Fn() -> Children + 'static,
    {
        Self {
            children: Box::new(children),
        }
    }
}

impl Widget for Row {
    fn build(&mut self, build_ctx: &mut BuildCtx) -> Children {
        (self.children)()
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        Some(constraints.max_size())
    }

    fn layout(&self, layout_ctx: &mut LayoutCtx, size: Size2D, children: &[usize]) {
        let child_sizes = children
            .iter()
            .map(|id| {
                (
                    *id,
                    layout_ctx.preferred_size(*id, &BoxConstraints::new(), layout_ctx),
                )
            })
            .collect::<Vec<(usize, Option<Size2D>)>>();

        let mut constrained_width = 0.0;
        let mut unconstrained_children = 0;
        for (id, child_size) in &child_sizes {
            if let Some(child_size) = child_size {
                layout_ctx.set_child_bounds(*id, Rect::new_from_size(*child_size));
                constrained_width += child_size.width;
            } else {
                unconstrained_children += 1;
            }
        }

        let left_over_width = size.width - constrained_width;
        let unconstrained_child_width = left_over_width / (unconstrained_children as f32).max(1.0);

        let mut x = 0.0;
        for (id, child_size) in &child_sizes {
            if let Some(child_size) = child_size {
                layout_ctx.set_child_position(*id, Point2D::new(x, 0.0));
                x += child_size.width;
            } else {
                layout_ctx.set_child_bounds(
                    *id,
                    Rect::new(
                        Point2D::new(x, 0.0),
                        Size2D::new(unconstrained_child_width, size.height),
                    ),
                );
            }
        }
    }
}
