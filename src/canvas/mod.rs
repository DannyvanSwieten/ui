use self::{color::Color, font::Font, paint::Paint, text::Text};
use crate::geo::{Point, Rect, Size};

pub mod canvas_renderer;
pub mod color;
pub mod font;
pub mod paint;
pub mod skia_cpu_canvas;
pub mod text;
pub trait Canvas: Send {
    fn clear(&mut self, color: &Color);

    fn save(&mut self);
    fn restore(&mut self);
    fn translate(&mut self, point: &Point);
    fn scale(&mut self, size: &Size);

    fn draw_rect(&mut self, rect: &Rect, paint: &Paint);
    fn draw_rounded_rect(&mut self, rect: &Rect, rx: f32, ry: f32, paint: &Paint);

    fn draw_circle(&mut self, center: &Point, radius: f32, paint: &Paint);

    fn draw_string(&mut self, rect: &Rect, text: &str, font: &Font, paint: &Paint);
    fn draw_text(&mut self, text: &Text, rect: &Rect, paint: &Paint);
    fn pixels(&mut self) -> Option<&[u8]>;
    fn clip_rect(&mut self, rect: &Rect);
    // fn draw_paragraph(&mut self, pos: &Point, paragraph: &Paragraph);
}
