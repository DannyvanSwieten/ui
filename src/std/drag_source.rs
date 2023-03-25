use crate::{
    constraints::BoxConstraints,
    event_context::EventCtx,
    geo::{Rect, Size},
    ui_state::UIState,
    widget::{BuildCtx, Child, Children, LayoutCtx, SizeCtx, Widget},
};
use std::any::Any;

pub struct DragSourceItem {
    widget: Box<dyn Widget>,
    _data: Option<Box<dyn Any>>,
}

impl DragSourceItem {
    pub fn widget(&self) -> &dyn Widget {
        self.widget.as_ref()
    }

    pub fn new(widget: Box<dyn Widget>) -> DragSourceItem {
        Self {
            widget,
            _data: None,
        }
    }
}

pub struct DragSourceData {
    items: Vec<DragSourceItem>,
}

impl DragSourceData {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn with_item(mut self, item: DragSourceItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn items(&self) -> &[DragSourceItem] {
        &self.items
    }
}

impl Default for DragSourceData {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DragSource {
    child: Child,
    drag_start: Option<Box<dyn Fn() -> DragSourceData>>,
}

impl DragSource {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn() -> Box<dyn Widget> + 'static,
    {
        Self {
            child: Box::new(child),
            drag_start: None,
        }
    }

    pub fn with_drag_start<F>(mut self, handler: F) -> Self
    where
        F: Fn() -> DragSourceData + 'static,
    {
        self.drag_start = Some(Box::new(handler));
        self
    }
}

impl Widget for DragSource {
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
        if let crate::event::MouseEvent::MouseDragStart(_mouse_event) = event_ctx.mouse_event() {
            // Register this component as drag source in ctx
            if let Some(handler) = &self.drag_start {
                event_ctx.set_drag_source(handler())
            }

            // If the DropTarget widget receives a MouseDrag event it may or may not signal to accept this widget by painting for example an outline.
            // If the DropTarget widget receives a MouseDragEnd event it then fires it's on_element_dropped callback.
        }
    }

    fn intercept_mouse_events(&self) -> bool {
        true
    }
}
