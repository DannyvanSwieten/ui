mod build_ctx;
pub mod constraints;
mod layout_ctx;
pub mod message_context;

pub use build_ctx::BuildCtx;
pub use layout_ctx::LayoutCtx;
pub use layout_ctx::SizeCtx;

use crate::user_interface::ui_state::UIState;
use crate::{event_context::EventCtx, geo::Size, painter::Painter};
use std::{any::Any, sync::Arc};

use self::constraints::BoxConstraints;
use self::message_context::MessageCtx;

pub type Child = Box<dyn Fn() -> Box<dyn Widget>>;
pub type Children = Vec<Box<dyn Widget>>;

pub enum ChangeResponse {
    Build,
    Layout,
    Paint,
}

#[allow(unused_variables)]
pub trait Widget {
    fn build(&self, build_ctx: &mut BuildCtx) -> Children {
        vec![]
    }

    fn state(&self, ui_state: &UIState) -> Option<Arc<dyn Any + Send>> {
        None
    }

    fn binding_changed(&self, _name: &str) -> Option<ChangeResponse> {
        None
    }

    fn calculate_size(
        &self,
        children: &[usize],
        constraints: &BoxConstraints,
        size_ctx: &SizeCtx,
    ) -> Option<Size> {
        None
    }

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
        _ui_state: &UIState,
        _event_ctx: &mut EventCtx,
        _message_ctx: &mut MessageCtx,
    ) {
    }

    fn animation_event(&self, event_context: &mut EventCtx, _ui_state: &UIState) {}
    fn intercept_mouse_events(&self) -> bool {
        false
    }
}

impl<T> From<T> for Box<dyn Widget>
where
    T: Widget + 'static,
{
    fn from(value: T) -> Self {
        Box::new(value)
    }
}
