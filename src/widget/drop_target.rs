use crate::{event_context::EventCtx, rect::Rect};

use super::{Child, Children, Widget};

pub struct DropTarget {
    child: Child,
}

impl DropTarget {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn() -> Box<dyn Widget> + 'static,
    {
        Self {
            child: Box::new(child),
        }
    }
}

impl Widget for DropTarget {
    fn build(&mut self, _build_ctx: &mut crate::build_context::BuildCtx) -> Children {
        vec![(self.child)()]
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &crate::constraints::BoxConstraints,
        layout_ctx: &crate::layout_ctx::LayoutCtx,
    ) -> Option<crate::size::Size2D> {
        layout_ctx.preferred_size(children[0], constraints, layout_ctx)
    }

    fn layout(
        &self,
        layout_ctx: &mut crate::layout_ctx::LayoutCtx,
        size: crate::size::Size2D,
        children: &[usize],
    ) {
        layout_ctx.set_child_bounds(children[0], Rect::new_from_size(size))
    }

    fn mouse_event(
        &mut self,
        event_ctx: &mut EventCtx,
        _message_ctx: &mut crate::message_context::MessageCtx,
    ) {
        if let crate::event::MouseEvent::MouseDrag(_mouse_event) = event_ctx.mouse_event() {}
    }

    fn intercept_mouse_events(&self) -> bool {
        true
    }
}
