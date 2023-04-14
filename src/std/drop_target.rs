use std::{any::Any, sync::Arc};

use crate::{
    app::event::MouseEvent,
    event_context::EventCtx,
    geo::{Rect, Size},
    user_interface::ui_state::UIState,
    widget::{
        constraints::BoxConstraints, message_context::MessageCtx, BuildCtx, Child, Children,
        LayoutCtx, SizeCtx, Widget,
    },
};

pub struct DropTarget<T> {
    child: Child,
    child_on_accept: Option<Child>,
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
            child_on_accept: None,
            accept: None,
            _data: std::marker::PhantomData::default(),
        }
    }

    pub fn with_accept<F>(mut self, accept: F) -> Self
    where
        F: Fn(&T) -> bool + 'static,
    {
        self.accept = Some(Box::new(accept));
        self
    }

    pub fn with_child_on_accept<C>(mut self, child: C) -> Self
    where
        C: Fn() -> Box<dyn Widget> + 'static,
    {
        self.child_on_accept = Some(Box::new(child));
        self
    }
}

struct DropTargetState {
    pub accepted: bool,
}

impl<T: Send + 'static> Widget for DropTarget<T> {
    fn state(&self, _: &UIState) -> Option<Arc<dyn Any + Send>> {
        Some(Arc::new(DropTargetState { accepted: false }))
    }

    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        let state = build_ctx.state::<DropTargetState>().unwrap();
        if state.accepted {
            if let Some(child) = &self.child_on_accept {
                vec![(child)()]
            } else {
                vec![(self.child)()]
            }
        } else {
            vec![(self.child)()]
        }
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
        _message_ctx: &mut MessageCtx,
    ) {
        if let MouseEvent::MouseDrag(_) = event_ctx.mouse_event() {
            if let Some(data) = event_ctx.drag_data::<T>() {
                if let Some(accept) = &self.accept {
                    if accept(data) {
                        event_ctx.set_state(|_| DropTargetState { accepted: true })
                    }
                }
            }
        }
    }

    fn intercept_mouse_events(&self) -> bool {
        true
    }
}
