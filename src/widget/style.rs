#[derive(Default, Clone, Copy)]
pub struct Insets {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Insets {
    pub fn all(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    pub fn horizontal(left: f32, right: f32) -> Self {
        Self {
            left,
            right,
            top: 0.0,
            bottom: 0.0,
        }
    }

    pub fn vertical(top: f32, bottom: f32) -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            top,
            bottom,
        }
    }
}
