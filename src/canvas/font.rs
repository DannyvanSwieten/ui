#[derive(Clone, Copy)]
pub enum FontStyle {
    Normal,
    Italic,
    Bold,
}

pub struct Font {
    typeface: String,
    style: FontStyle,
    size: f32,
}

impl Font {
    pub fn size(&self) -> f32 {
        self.size
    }

    pub fn typeface(&self) -> &str {
        &self.typeface
    }

    pub fn style(&self) -> FontStyle {
        self.style
    }
}
