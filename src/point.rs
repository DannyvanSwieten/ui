#[derive(Clone, Copy)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub<Point2D> for Point2D {
    type Output = Self;

    fn sub(self, rhs: Point2D) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add<Point2D> for Point2D {
    type Output = Self;

    fn add(self, rhs: Point2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
