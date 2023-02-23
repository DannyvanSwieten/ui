use crate::{
    build_context::BuildCtx, constraints::BoxConstraints, layout_ctx::LayoutCtx, point::Point2D,
    rect::Rect, size::Size2D, ui_state::UIState, value::Value,
};

type Children = Vec<Box<dyn Widget>>;

pub trait Widget {
    fn build(&mut self, build_ctx: &mut BuildCtx) -> Option<Children> {
        None
    }
    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<(f32, f32)> {
        None
    }

    fn layout(&self, layout_ctx: &mut LayoutCtx, size: Size2D, children: &[usize]) {}
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
    fn build(&mut self, build_ctx: &mut BuildCtx) -> Option<Children> {
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
    ) -> Option<(f32, f32)> {
        Some((100.0, 50.0))
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

impl Widget for Center {
    fn build(&mut self, build_ctx: &mut BuildCtx) -> Option<Children> {
        Some(vec![(*self.child)()])
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<(f32, f32)> {
        // Something, Somewhere, went terribly wrong
        assert_eq!(1, children.len());

        // Return all the space that is given to this widget.
        Some((
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        ))
    }

    fn layout(&self, layout_ctx: &mut LayoutCtx, size: Size2D, children: &[usize]) {
        // Something, Somewhere, went terribly wrong
        assert_eq!(1, children.len());

        let (center_x, center_y) = (size.width / 2.0, size.height / 2.0);
        let child_size = if let Some((width, height)) = layout_ctx.preferred_size(
            children[0],
            &BoxConstraints::new_with_max(size.width, size.height),
            layout_ctx,
        ) {
            Size2D::new(width, height)
        } else {
            size
        };

        let position = Point2D {
            x: center_x - child_size.width / 2.0,
            y: center_y - child_size.height / 2.0,
        };

        layout_ctx.set_child_rect(children[0], Rect::new(position, child_size))
    }
}
