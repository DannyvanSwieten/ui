use crate::{point::Point2D, rect::Rect};

use self::{color::Color, font::Font, paint::Paint};

pub mod canvas_renderer;
pub mod color;
pub mod font;
pub mod paint;
pub mod paint_ctx;
pub mod skia_cpu_canvas;
pub trait Canvas2D {
    fn clear(&mut self, color: &Color);

    fn save(&mut self);
    fn restore(&mut self);
    fn translate(&mut self, point: &Point2D);

    fn draw_rect(&mut self, rect: &Rect, paint: &Paint);
    fn draw_rounded_rect(&mut self, rect: &Rect, rx: f32, ry: f32, paint: &Paint);

    fn draw_circle(&mut self, center: &Point2D, radius: f32, paint: &Paint);

    fn draw_string(&mut self, rect: &Rect, text: &str, font: &Font, paint: &Paint);
    fn pixels(&mut self) -> Option<&[u8]>;
    // fn draw_text_blob(&mut self, pos: &Point2D, blob: &TextBlob, paint: &Paint);
    // fn draw_paragraph(&mut self, pos: &Point, paragraph: &Paragraph);
}
