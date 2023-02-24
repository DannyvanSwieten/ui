use super::color::Color;

pub struct Paint {
    color: Color,
}

impl Paint {
    pub fn new(color: impl Into<Color>) -> Self {
        Self {
            color: color.into(),
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
}
