use crate::geo::Rect;
use std::any::Any;

pub struct PaintCtx<'a> {
    global_bounds: &'a Rect,
    local_bounds: &'a Rect,
    state: Option<&'a (dyn Any + Send)>,
}

impl<'a> PaintCtx<'a> {
    pub fn new(
        global_bounds: &'a Rect,
        local_bounds: &'a Rect,
        state: Option<&'a (dyn Any + Send)>,
    ) -> Self {
        Self {
            global_bounds,
            local_bounds,
            state,
        }
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
            state.downcast_ref::<T>()
        } else {
            None
        }
    }
}
