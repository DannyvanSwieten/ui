use std::rc::Rc;

use crate::{
    geo::{Point, Rect, Size},
    user_interface::ui_state::UIState,
    widget::{constraints::BoxConstraints, BuildCtx, Child, Children, LayoutCtx, SizeCtx, Widget},
};

pub struct Center {
    child: Child,
}

impl Center {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn(&UIState) -> Box<dyn Widget> + 'static,
    {
        Self {
            child: Rc::new(child),
        }
    }
}

impl Widget for Center {
    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        vec![(*self.child)(build_ctx.ui_state())]
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        _: &SizeCtx,
    ) -> Option<Size> {
        // It needs exactly one child
        assert_eq!(1, children.len());
        // If no max width or height is given, we can't center the child
        assert!(constraints.max_width().is_some());
        assert!(constraints.max_height().is_some());

        // Return all the space that is given to this widget.
        Some(Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        ))
    }

    fn layout(
        &self,
        _ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
        // Calculate our center
        let (center_x, center_y) = (size.width / 2.0, size.height / 2.0);

        // Ask the child for its preferred size given this bounds as constraints
        let child_size = if let Some(child_size) = layout_ctx.preferred_size(
            children[0],
            &BoxConstraints::new_with_max(size.width, size.height),
        ) {
            child_size
        } else {
            size
        };

        // Position the child in the center
        let position = Point {
            x: center_x - child_size.width / 2.0,
            y: center_y - child_size.height / 2.0,
        };

        layout_ctx.set_child_bounds(children[0], Rect::new(position, child_size))
    }
}
