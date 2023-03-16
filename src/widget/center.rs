use crate::{
    build_context::BuildCtx,
    constraints::BoxConstraints,
    geo::{Point2D, Rect, Size2D},
    layout_ctx::LayoutCtx,
    ui_state::UIState,
};

use super::{Child, Children, Widget};

pub struct Center {
    child: Child,
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
    fn build(&self, _build_ctx: &mut BuildCtx) -> Children {
        vec![(*self.child)()]
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

    fn layout(
        &self,
        _ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size2D,
        children: &[usize],
    ) {
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
