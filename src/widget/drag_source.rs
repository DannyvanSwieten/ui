use crate::{event_context::EventCtx, rect::Rect};

use super::{Child, Widget};

pub struct DragSource {
    child: Child,
}

impl DragSource {
    pub fn new(child: Child) -> Self {
        Self { child }
    }
}

impl Widget for DragSource {
    fn build(&mut self, _build_ctx: &mut crate::build_context::BuildCtx) -> super::Children {
        Some(vec![(self.child)()])
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
        message_ctx: &mut crate::message_context::MessageCtx,
    ) {
        if let crate::event::MouseEvent::MouseDragStart(_mouse_event) = event_ctx.mouse_event() {
            // Register this component as drag source in ctx
            event_ctx.set_drag_source()
            // If the DragTarget widget receives a MouseDrag event it may or may not signal to accept this widget by painting for example an outline.
            // If the DragTarget widget receives a MouseDragEnd event it then fires it's on_element_dropped callback.
        }
    }
}
