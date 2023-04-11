use std::ops::{Div, Mul};

#[derive(Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn uniform(u: f32) -> Self {
        Self {
            width: u,
            height: u,
        }
    }
}

impl Mul<f32> for Size {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl Div<f32> for Size {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}
