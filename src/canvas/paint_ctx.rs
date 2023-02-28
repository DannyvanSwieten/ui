use crate::rect::Rect;

pub struct PaintCtx<'a> {
    global_bounds: &'a Rect,
    local_bounds: &'a Rect,
}

impl<'a> PaintCtx<'a> {
    pub fn new(global_bounds: &'a Rect, local_bounds: &'a Rect) -> Self {
        Self {
            global_bounds,
            local_bounds,
        }
    }

    pub fn global_bounds(&self) -> &'a Rect {
        self.global_bounds
    }

    pub fn local_bounds(&self) -> &'a Rect {
        self.local_bounds
    }
}
