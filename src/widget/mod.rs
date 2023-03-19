mod build_ctx;
mod layout_ctx;
mod widget_tree;

pub use build_ctx::BuildCtx;
pub use layout_ctx::LayoutCtx;
pub use widget_tree::{WidgetElement, WidgetTree};

use crate::{
    constraints::BoxConstraints, event_context::EventCtx, geo::Size, message_context::MessageCtx,
    painter::Painter, ui_state::UIState,
};
use std::any::Any;

pub type Child = Box<dyn Fn() -> Box<dyn GenericWidget>>;
pub type Children = Vec<Box<dyn GenericWidget>>;

pub trait GenericWidget {
    fn build(&self, build_ctx: &mut BuildCtx) -> Children;

    fn state(&self) -> Option<Box<dyn Any + Send>>;

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size>;

    fn layout(
        &self,
        ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    );

    fn painter(&self, ui_state: &UIState) -> Option<Box<dyn Painter>>;

    fn mouse_event(
        &self,
        _ui_state: &UIState,
        _event_ctx: &mut EventCtx,
        _message_ctx: &mut MessageCtx,
    );

    fn intercept_mouse_events(&self) -> bool;
}

impl<T> From<T> for Box<dyn GenericWidget>
where
    T: GenericWidget + 'static,
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

#[allow(unused_variables)]
pub trait Widget {
    type State: Any + Send;

    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        vec![]
    }

    fn state(&self) -> Option<Self::State> {
        None
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size>;

    fn layout(
        &self,
        ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
    }

    fn painter(&self, ui_state: &UIState) -> Option<Box<dyn Painter>> {
        None
    }

    fn mouse_event(
        &self,
        ui_state: &UIState,
        event_ctx: &mut EventCtx,
        message_ctx: &mut MessageCtx,
    ) {
    }

    fn intercept_mouse_events(&self) -> bool {
        false
    }
}

impl<T> GenericWidget for T
where
    T: Widget,
{
    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        self.build(build_ctx)
    }

    fn state(&self) -> Option<Box<dyn Any + Send>> {
        self.state()
            .map(|state| Box::new(state) as Box<dyn Any + Send>)
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size> {
        self.calculate_size(children, constraints, layout_ctx)
    }

    fn layout(
        &self,
        ui_state: &UIState,
        layout_ctx: &mut LayoutCtx,
        size: Size,
        children: &[usize],
    ) {
        self.layout(ui_state, layout_ctx, size, children)
    }

    fn painter(&self, ui_state: &UIState) -> Option<Box<dyn Painter>> {
        self.painter(ui_state)
    }

    fn mouse_event(
        &self,
        ui_state: &UIState,
        event_ctx: &mut EventCtx,
        message_ctx: &mut MessageCtx,
    ) {
        self.mouse_event(ui_state, event_ctx, message_ctx)
    }

    fn intercept_mouse_events(&self) -> bool {
        self.intercept_mouse_events()
    }
}
