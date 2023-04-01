use crate::{
    geo::{Point, Rect, Size},
    widget::{Child, Widget},
};
/// A widget that has a fixed size. It will not violate maximum or minimum constraints.
/// It will force its child to have the same size.
/// It can be used to determine a fixed amount of space between two widgets.
pub struct SizedBox {
    size: Size,
    child: Option<Child>,
}

impl SizedBox {
    pub fn new(size: Size) -> Self {
        Self { size, child: None }
    }

    pub fn with_child(mut self, child: Child) -> Self {
        self.child = Some(child);
        self
    }
}

impl Widget for SizedBox {
    fn build(&self, _: &mut crate::widget::BuildCtx) -> crate::widget::Children {
        if let Some(child) = &self.child {
            vec![child()]
        } else {
            vec![]
        }
    }

    fn calculate_size(
        &self,
        _: &[usize],
        constraints: &crate::constraints::BoxConstraints,
        _: &crate::widget::SizeCtx,
    ) -> Option<Size> {
        let width = constraints
            .max_width()
            .map(|max_width| max_width.min(self.size.width))
            .unwrap_or(self.size.width);

        let height = constraints
            .max_height()
            .map(|max_height| max_height.min(self.size.height))
            .unwrap_or(self.size.height);

        Some(Size::new(width, height))
    }

    fn layout(
        &self,
        _: &crate::ui_state::UIState,
        layout_ctx: &mut crate::widget::LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
        if children.len() == 1 {
            layout_ctx.set_child_bounds(children[0], Rect::new(Point::new(0.0, 0.0), size));
        }
    }
}
