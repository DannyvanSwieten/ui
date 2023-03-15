use std::any::Any;

use crate::{rect::Rect, ui_state::UIState};

pub struct PaintCtx<'a> {
    global_bounds: &'a Rect,
    local_bounds: &'a Rect,
    state: &'a Option<Box<dyn Any>>,
    ui_state: &'a UIState,
}

impl<'a> PaintCtx<'a> {
    pub fn new(
        global_bounds: &'a Rect,
        local_bounds: &'a Rect,
        state: &'a Option<Box<dyn Any>>,
        ui_state: &'a UIState,
    ) -> Self {
        Self {
            global_bounds,
            local_bounds,
            state,
            ui_state,
        }
    }

    pub fn ui_state(&self) -> &'a UIState {
        self.ui_state
    }

    pub fn global_bounds(&self) -> &'a Rect {
        self.global_bounds
    }

    pub fn local_bounds(&self) -> &'a Rect {
        self.local_bounds
    }

    pub fn state<T>(&self) -> Option<&T>
    where
        T: 'static,
    {
        if let Some(state) = self.state {
            state.as_ref().downcast_ref::<T>()
        } else {
            None
        }
    }
}
