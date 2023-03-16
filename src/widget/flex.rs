use crate::{
    build_context::BuildCtx,
    constraints::BoxConstraints,
    geo::{Point2D, Rect, Size2D},
    layout_ctx::LayoutCtx,
    ui_state::UIState,
};

use super::{Children, Widget};

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
    fn build(&self, _build_ctx: &mut BuildCtx) -> Children {
        (self.children)()
    }

    fn calculate_size(
        &self,
        _children: &[usize],
        constraints: &BoxConstraints,
        _layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        Some(constraints.max_size())
    }

    fn layout(
        &self,
        _ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size2D,
        children: &[usize],
    ) {
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
                layout_ctx.set_child_position(
                    *id,
                    Point2D::new(x, size.height / 2.0 - child_size.height / 2.0),
                );
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
