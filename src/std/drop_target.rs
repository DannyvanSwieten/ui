use crate::{
    constraints::BoxConstraints,
    event_context::EventCtx,
    geo::{Rect, Size},
    ui_state::UIState,
    widget::{BuildCtx, Child, Children, GenericWidget, LayoutCtx, Widget},
};

pub struct DropTarget {
    child: Child,
}

impl DropTarget {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn() -> Box<dyn GenericWidget> + 'static,
    {
        Self {
            child: Box::new(child),
        }
    }
}

impl Widget for DropTarget {
    type State = ();

    fn build(&self, _build_ctx: &mut BuildCtx) -> Children {
        vec![(self.child)()]
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size> {
        layout_ctx.preferred_size(children[0], constraints, layout_ctx)
    }

    fn layout(
        &self,
        _ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
        layout_ctx.set_child_bounds(children[0], Rect::new_from_size(size))
    }

    fn mouse_event(
        &self,
        _ui_state: &UIState,
        event_ctx: &mut EventCtx,
        _message_ctx: &mut crate::message_context::MessageCtx,
    ) {
        if let crate::event::MouseEvent::MouseDrag(_mouse_event) = event_ctx.mouse_event() {}
    }

    fn intercept_mouse_events(&self) -> bool {
        true
    }
}
