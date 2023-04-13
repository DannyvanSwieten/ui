use std::{any::Any, sync::Arc};

use crate::{
    app::event::MouseEvent,
    canvas::{color::Color32f, paint::Paint},
    event_context::EventCtx,
    geo::{Rect, Size},
    painter::Painter,
    user_interface::ui_state::UIState,
    widget::{
        constraints::BoxConstraints, message_context::MessageCtx, BuildCtx, Child, Children,
        LayoutCtx, SizeCtx, Widget,
    },
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

struct DropTargetState {
    pub accepted: bool,
}

impl<T: Send + 'static> Widget for DropTarget<T> {
    fn state(&self, _: &UIState) -> Option<Arc<dyn Any + Send>> {
        Some(Arc::new(DropTargetState { accepted: false }))
    }

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
        _message_ctx: &mut MessageCtx,
    ) {
        if let MouseEvent::MouseDrag(mouse_event) = event_ctx.mouse_event() {
            if let Some(data) = mouse_event.drag_data::<T>() {
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

    fn painter(&self, _: &UIState) -> Option<Box<dyn Painter>> {
        Some(Box::new(DropTargetPainter::<T> {
            _data: std::marker::PhantomData::default(),
        }))
    }
}

pub struct DropTargetPainter<T: 'static> {
    _data: std::marker::PhantomData<T>,
}
impl<T: Send + 'static> Painter for DropTargetPainter<T> {
    fn paint(&self, paint_ctx: &crate::painter::PaintCtx, canvas: &mut dyn crate::canvas::Canvas) {
        if let Some(state) = paint_ctx.state::<DropTargetState>() {
            if state.accepted {
                let paint = Paint::new(Color32f::new(0.0, 1.0, 0.0, 0.5));
                canvas.draw_rect(paint_ctx.local_bounds(), &paint);
            }
        }
    }
}
