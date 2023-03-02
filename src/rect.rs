use crate::{point::Point2D, size::Size2D};
#[derive(Default, Clone, Copy)]
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
            right: position.x + size.width,
            bottom: position.y + size.height,
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

    pub fn set_position(&mut self, position: Point2D) {
        let size = self.size();
        self.left = position.x;
        self.right = position.x + size.width;
        self.top = position.y;
        self.bottom = position.y + size.height;
    }

    pub fn position(&self) -> Point2D {
        Point2D {
            x: self.left,
            y: self.top,
        }
    }

    pub fn size(&self) -> Size2D {
        Size2D {
            width: self.right - self.left,
            height: self.bottom - self.top,
        }
    }

    pub fn hit_test(&self, point: &Point2D) -> bool {
        point.x >= self.left && point.x < self.right && point.y >= self.top && point.y < self.bottom
    }

    pub fn with_offset(mut self, point: Point2D) -> Self {
        self.left += point.x;
        self.right += point.x;
        self.top += point.y;
        self.bottom += point.y;
        self
    }
}
