use crate::{
    geo::{Point, Rect, Size},
    user_interface::ui_state::UIState,
    widget::{constraints::BoxConstraints, BuildCtx, Children, LayoutCtx, SizeCtx, Widget},
};

pub struct List {
    children: Box<dyn Fn() -> Children>,
}

impl List {
    pub fn new<C>(children: C) -> Self
    where
        C: Fn() -> Children + 'static,
    {
        Self {
            children: Box::new(children),
        }
    }
}

impl Widget for List {
    fn build(&self, _build_ctx: &mut BuildCtx) -> Children {
        (self.children)()
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        size_ctx: &SizeCtx,
    ) -> Option<Size> {
        let width = constraints.max_width().unwrap();
        let mut height = 0.0;
        let child_constraints = BoxConstraints::new().with_max_width(width);
        for child in children {
            let child_size = size_ctx.preferred_size(*child, &child_constraints);
            if let Some(child_size) = child_size {
                height += child_size.height;
            }
        }

        Some(Size::new(width, height))
    }

    fn layout(
        &self,
        _ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
        let mut y = 0.0;
        let child_constraints = BoxConstraints::new().with_max_width(size.width);
        for child in children {
            if let Some(child_size) = layout_ctx.preferred_size(*child, &child_constraints) {
                layout_ctx.set_child_bounds(
                    *child,
                    Rect::new(Point::new(0.0, y), Size::new(size.width, child_size.height)),
                );
                y += child_size.height;
            }
        }
    }
}
