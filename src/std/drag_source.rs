use crate::{
    app::event::MouseEvent,
    event_context::EventCtx,
    geo::{Point, Rect, Size},
    user_interface::ui_state::UIState,
    widget::{
        constraints::BoxConstraints, message_context::MessageCtx, BuildCtx, Child, Children,
        LayoutCtx, SizeCtx, Widget,
    },
};

pub struct DragSource<T> {
    child: Child,
    child_when_dragging: Option<Child>,
    dragging_child: Option<Child>,
    drag_start: Option<Box<dyn Fn() -> T>>,
}

impl<T> DragSource<T> {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn() -> Box<dyn Widget> + 'static,
    {
        Self {
            child: Box::new(child),
            drag_start: None,
            child_when_dragging: None,
            dragging_child: None,
        }
    }

    pub fn with_drag_start<F>(mut self, handler: F) -> Self
    where
        F: Fn() -> T + 'static,
    {
        self.drag_start = Some(Box::new(handler));
        self
    }

    pub fn with_child_when_dragging<C>(mut self, child: C) -> Self
    where
        C: Fn() -> Box<dyn Widget> + 'static,
    {
        self.child_when_dragging = Some(Box::new(child));
        self
    }

    pub fn with_dragging_child<C>(mut self, child: C) -> Self
    where
        C: Fn() -> Box<dyn Widget> + 'static,
    {
        self.dragging_child = Some(Box::new(child));
        self
    }
}

#[derive(Clone, Copy)]
struct DragState {
    pub dragging: bool,
    pub position: Point,
}

impl<T: 'static> Widget for DragSource<T> {
    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        let state = build_ctx.state::<DragState>().unwrap();
        let child = if let Some(c) = &self.child_when_dragging {
            if state.dragging {
                (c)()
            } else {
                (self.child)()
            }
        } else {
            (self.child)()
        };

        vec![child]
    }

    fn state(&self, _: &UIState) -> Option<std::sync::Arc<dyn std::any::Any + Send>> {
        Some(std::sync::Arc::new(DragState {
            dragging: false,
            position: Point::new(0.0, 0.0),
        }))
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
        let child_size = layout_ctx
            .preferred_size(
                children[0],
                &BoxConstraints::new_with_max(size.width, size.height),
            )
            .unwrap_or(size);

        layout_ctx.set_child_bounds(children[0], Rect::new_from_size(child_size));
    }

    fn mouse_event(
        &self,
        _ui_state: &UIState,
        event_ctx: &mut EventCtx,
        _message_ctx: &mut MessageCtx,
    ) {
        if let MouseEvent::MouseDragStart(mouse_event) = event_ctx.mouse_event() {
            // Register this component as drag source in ctx
            if let Some(handler) = &self.drag_start {
                if let Some(dragging_child) = &self.dragging_child {
                    event_ctx.set_drag_source(dragging_child(), handler())
                } else {
                    event_ctx.set_drag_source((self.child)(), handler())
                }
            }
            let position = *mouse_event.local_position();
            event_ctx.set_state(move |_| DragState {
                dragging: true,
                position,
            });

            // If the DropTarget widget receives a MouseDrag event it may or may not signal to accept this widget by painting for example an outline.
            // If the DropTarget widget receives a MouseDragEnd event it then fires it's on_element_dropped callback.
        }

        if let MouseEvent::MouseDrag(mouse_event) = event_ctx.mouse_event() {
            // Register this component as drag source in ctx
            if let Some(handler) = &self.drag_start {
                if let Some(dragging_child) = &self.dragging_child {
                    event_ctx.set_drag_source(dragging_child(), handler())
                } else {
                    event_ctx.set_drag_source((self.child)(), handler())
                }
            }
            let position = *mouse_event.local_position();
            event_ctx.set_state(move |_| DragState {
                dragging: true,
                position,
            });

            // If the DropTarget widget receives a MouseDrag event it may or may not signal to accept this widget by painting for example an outline.
            // If the DropTarget widget receives a MouseDragEnd event it then fires it's on_element_dropped callback.
        }

        if let MouseEvent::MouseDragEnd(_) = event_ctx.mouse_event() {
            // Register this component as drag source in ctx
            if let Some(handler) = &self.drag_start {
                if let Some(dragging_child) = &self.dragging_child {
                    event_ctx.set_drag_source(dragging_child(), handler())
                } else {
                    event_ctx.set_drag_source((self.child)(), handler())
                }
            }
            let position = Point::default();
            event_ctx.set_state(move |_| DragState {
                dragging: false,
                position,
            });

            // If the DropTarget widget receives a MouseDrag event it may or may not signal to accept this widget by painting for example an outline.
            // If the DropTarget widget receives a MouseDragEnd event it then fires it's on_element_dropped callback.
        }
    }

    fn intercept_mouse_events(&self) -> bool {
        true
    }
}
