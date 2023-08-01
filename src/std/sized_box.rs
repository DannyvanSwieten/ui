use std::rc::Rc;

use crate::{
    geo::{Point, Rect, Size},
    user_interface::ui_state::UIState,
    widget::{constraints::BoxConstraints, Child, LayoutCtx, Widget},
};
/// A widget that has a fixed size. It will not violate maximum or minimum constraints.
/// It will force its child to have the same size.
/// It can be used to determine a fixed amount of space between two widgets.
pub struct SizedBox {
    size: Size,
    child: Option<Child>,
}

pub fn sized_box(size: Size) -> Box<SizedBox> {
    Box::new(SizedBox::new(size))
}

impl SizedBox {
    pub fn new(size: Size) -> Self {
        Self { size, child: None }
    }

    pub fn with_child<C>(mut self, child: C) -> Self
    where
        C: Fn(&UIState) -> Box<dyn Widget> + 'static,
    {
        self.child = Some(Rc::new(child));
        self
    }
}

impl Widget for SizedBox {
    fn build(&self, build_ctx: &mut crate::widget::BuildCtx) -> crate::widget::Children {
        if let Some(child) = &self.child {
            vec![child(build_ctx.ui_state())]
        } else {
            vec![]
        }
    }

    fn calculate_size(
        &self,
        _: &[usize],
        constraints: &BoxConstraints,
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

    fn layout(&self, _: &UIState, layout_ctx: &mut LayoutCtx, size: Size, children: &[usize]) {
        if children.len() == 1 {
            layout_ctx.set_child_bounds(children[0], Rect::new(Point::new(0.0, 0.0), size));
        }
    }
}
