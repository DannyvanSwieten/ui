use crate::{
    constraints::BoxConstraints,
    event_context::EventCtx,
    geo::{Rect, Size},
    ui_state::UIState,
    widget::{BuildCtx, Child, Children, LayoutCtx, SizeCtx, Widget},
};

pub struct DropTarget<T> {
    child: Child,
    accept: Option<Box<dyn Fn(&T) -> bool>>,
    _data: std::marker::PhantomData<T>,
}

impl<T> DropTarget<T> {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn() -> Box<dyn Widget> + 'static,
    {
        Self {
            child: Box::new(child),
            accept: None,
            _data: std::marker::PhantomData::default(),
        }
    }
}

impl<T: 'static> Widget for DropTarget<T> {
    fn build(&self, _build_ctx: &mut BuildCtx) -> Children {
        vec![(self.child)()]
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        size_ctx: &SizeCtx,
    ) -> Option<Size> {
        size_ctx.preferred_size(children[0], constraints)
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
        if let crate::event::MouseEvent::MouseDrag(mouse_event) = event_ctx.mouse_event() {
            if let Some(data) = mouse_event.drag_data::<T>() {
                if let Some(accept) = &self.accept {
                    accept(data);
                }
            }
        }
    }

    fn intercept_mouse_events(&self) -> bool {
        true
    }
}
