use std::{any::Any, rc::Rc, sync::Arc};

use crate::{
    app::event::MouseEvent,
    canvas::Canvas,
    event_context::EventCtx,
    geo::{Point, Rect, Size},
    painter::{PaintCtx, Painter},
    user_interface::{ui_ctx::UIContext, ui_state::UIState},
    widget::{
        constraints::BoxConstraints, message_context::ApplicationCtx, ui_message::UIMessage,
        BuildCtx, Child, Children, LayoutCtx, SizeCtx, Widget,
    },
};

pub struct Viewport {
    child: Child,
}
pub struct ViewportState {
    offset: Point,
}
pub struct Scrollable {
    child: Child,
    speed: f32,
}

impl Scrollable {
    pub fn new<C>(child: C) -> Self
    where
        C: Fn(&UIState) -> Box<dyn Widget> + 'static,
    {
        Self {
            child: Rc::new(child),
            speed: 5.0,
        }
    }
}

impl Widget for Scrollable {
    fn build(&self, _build_ctx: &mut BuildCtx) -> Children {
        vec![Viewport::new(self.child.clone()).into()]
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        _size_ctx: &SizeCtx,
    ) -> Option<Size> {
        // It needs exactly one child
        assert_eq!(1, children.len());
        // If no max width or height is given, we can't center the child
        assert!(constraints.max_width().is_some());
        assert!(constraints.max_height().is_some());

        // Return all the space that is given to this widget.
        Some(Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        ))
    }

    fn layout(
        &self,
        _ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
        layout_ctx.set_child_bounds(children[0], Rect::new(Point::new(0.0, 0.0), size));
    }

    fn mouse_event(
        &self,
        _ui_state: &UIState,
        event_ctx: &mut EventCtx,
        ui_ctx: &mut UIContext,
        _message_ctx: &mut ApplicationCtx,
    ) {
        if let MouseEvent::MouseScroll(event) = event_ctx.mouse_event() {
            let scroll = event.scroll();
            ui_ctx.send_internal_message(
                UIMessage::new(ui_ctx.id(), ui_ctx.child_id(0), "set_offset")
                    .with_args(vec![scroll.x * self.speed, scroll.y * self.speed]),
            )
        }
    }

    fn intercept_mouse_events(&self) -> bool {
        true
    }
}

pub struct ViewportPainter {}

impl Painter for ViewportPainter {
    fn paint(&self, paint_ctx: &PaintCtx, canvas: &mut dyn Canvas) {
        canvas.clip_rect(paint_ctx.local_bounds())
    }
}

impl Viewport {
    pub fn new(child: Child) -> Self {
        Self { child }
    }
}

impl Widget for Viewport {
    fn state(&self, _ui_state: &UIState) -> Option<Arc<dyn Any + Send>> {
        Some(Arc::new(ViewportState {
            offset: Point::new(0.0, 0.0),
        }))
    }

    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        vec![(*self.child)(build_ctx.ui_state())]
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        _: &SizeCtx,
    ) -> Option<Size> {
        // It needs exactly one child
        assert_eq!(1, children.len());
        // If no max width or height is given, we can't center the child
        assert!(constraints.max_width().is_some());
        assert!(constraints.max_height().is_some());

        // Return all the space that is given to this widget.
        Some(Size::new(
            constraints.max_width().unwrap(),
            constraints.max_height().unwrap(),
        ))
    }

    // The viewport will assign an offset and make the child as big as it wants to be or its own size if no preferred size is returned.
    fn layout(
        &self,
        _ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
        let child_size = layout_ctx.preferred_size(
            children[0],
            &BoxConstraints::new()
                .with_max_width(size.width)
                .with_max_height(size.height),
        );
        if let Some(state) = layout_ctx.state::<ViewportState>() {
            layout_ctx.set_child_bounds(
                children[0],
                Rect::new(state.offset, child_size.unwrap_or(size)),
            );
        }
    }

    fn internal_event(
        &self,
        event_context: &mut EventCtx,
        ui_ctx: &mut UIContext,
        _ui_state: &UIState,
    ) {
        let message = event_context.ui_message();
        if message.target == "set_offset" {
            assert_eq!(message.args.len(), 2);
            let x = message.args[0].as_real().unwrap();
            let y = message.args[1].as_real().unwrap();
            ui_ctx.set_state::<ViewportState>(move |old_state| ViewportState {
                offset: old_state.offset + Point::new(x, y),
            })
        }
    }

    fn painter(&self, _ui_state: &UIState) -> Option<Box<dyn Painter>> {
        Some(Box::new(ViewportPainter {}))
    }
}
