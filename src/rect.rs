use crate::{point::Point2D, size::Size2D};
#[derive(Default)]
pub struct Rect {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl Rect {
    pub fn new(position: Point2D, size: Size2D) -> Self {
        Self {
            left: position.x,
            top: position.y,
            right: size.width,
            bottom: size.height,
        }
    }

    pub fn new_from_size(size: Size2D) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: size.width,
            bottom: size.height,
        }
    }

    pub fn size(&self) -> Size2D {
        Size2D {
            width: self.right - self.left,
            height: self.bottom - self.top,
        }
    }
}
