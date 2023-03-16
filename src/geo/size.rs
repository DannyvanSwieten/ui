#[derive(Clone, Copy)]
pub struct Size2D {
    pub width: f32,
    pub height: f32,
}
impl Size2D {
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
