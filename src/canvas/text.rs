use skia_safe::TextBlob;

use crate::geo::{Point, Rect, Size};

use super::font::Font;

pub struct Text {
    text: String,
    blob: TextBlob,
    font: Font,
}

impl Text {
    pub fn new(text: &str, font: Font) -> Self {
        let blob = skia_safe::TextBlob::new(text, &(&font).into()).unwrap();
        Self {
            text: text.into(),
            blob,
            font,
        }
    }

    pub fn bounds(&self) -> Rect {
        let bounds = self.blob.bounds();
        Rect::new(
            Point::new(bounds.left(), bounds.top()),
            Size::new(bounds.width(), bounds.height()),
        )
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn font(&self) -> &Font {
        &self.font
    }
}
