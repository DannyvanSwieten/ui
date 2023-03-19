use crate::{
    constraints::BoxConstraints,
    geo::{Point, Rect, Size},
    ui_state::UIState,
    widget::{BuildCtx, Child, Children, GenericWidget, LayoutCtx, Widget},
};

pub struct Center {
    child: Child,
}

impl Center {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn() -> Box<dyn GenericWidget> + 'static,
    {
        Self {
            child: Box::new(child),
        }
    }
}

impl Widget for Center {
    type State = ();

    fn build(&self, _build_ctx: &mut BuildCtx) -> Children {
        vec![(*self.child)()]
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        _: &LayoutCtx,
    ) -> Option<Size> {
        // Something, Somewhere, went terribly wrong
        assert_eq!(1, children.len());

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

        let position = Point {
            x: center_x - child_size.width / 2.0,
            y: center_y - child_size.height / 2.0,
        };

        layout_ctx.set_child_bounds(children[0], Rect::new(position, child_size))
    }
}
