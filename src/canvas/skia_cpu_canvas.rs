use super::{color::Color, font::Font, paint::Paint, Canvas};
use crate::geo::{self, Rect, Size};
use skia_safe::{utils::text_utils::Align, ISize, Point, Surface, TextBlob};

pub struct SkiaCanvas {
    surface: Surface,
    pixels: Vec<u8>,
    pub size: ISize,
}

impl SkiaCanvas {
    pub fn new(w: i32, h: i32) -> Self {
        let surface = Surface::new_raster_n32_premul(skia_safe::ISize::new(w, h));
        let mut pixels = Vec::new();
        pixels.resize(4 * w as usize * h as usize, 0);
        if let Some(surface) = surface {
            Self {
                surface,
                size: skia_safe::ISize::new(w, h),
                pixels,
            }
        } else {
            panic!()
        }
    }

    pub fn pixels(&mut self) -> Option<&[u8]> {
        self.surface.flush_and_submit();
        let w = self.surface.width();
        let info = self.surface.image_info();
        if self.surface.read_pixels(
            &info,
            &mut self.pixels,
            w as usize * 4,
            skia_safe::IPoint::new(0, 0),
        ) {
            Some(&self.pixels)
        } else {
            None
        }
    }

    pub fn flush(&mut self) {
        self.surface.flush_and_submit();
    }
}

impl From<geo::Point> for skia_safe::Point {
    fn from(val: geo::Point) -> Self {
        skia_safe::Point::new(val.x, val.y)
    }
}

impl From<Size> for skia_safe::Size {
    fn from(val: Size) -> Self {
        skia_safe::Size::new(val.width, val.height)
    }
}

impl From<Rect> for skia_safe::Rect {
    fn from(val: Rect) -> Self {
        skia_safe::Rect::from_point_and_size(val.position(), val.size())
    }
}

impl From<&geo::Point> for skia_safe::Point {
    fn from(value: &geo::Point) -> Self {
        skia_safe::Point {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<&Rect> for skia_safe::Rect {
    fn from(value: &Rect) -> Self {
        skia_safe::Rect::from_point_and_size(value.position(), value.size())
    }
}

impl From<&Color> for skia_safe::Color4f {
    fn from(value: &Color) -> Self {
        let [r, g, b, a] = value.as_floats();
        skia_safe::Color4f::new(r, g, b, a)
    }
}

impl From<&Paint> for skia_safe::Paint {
    fn from(value: &Paint) -> Self {
        let [r, g, b, a] = value.color().as_floats();
        let mut p = skia_safe::Paint::new(skia_safe::Color4f::new(r, g, b, a), None);
        p.set_anti_alias(true);
        p
    }
}

impl From<&Font> for skia_safe::Font {
    fn from(value: &Font) -> Self {
        let mut font = skia_safe::Font::new(
            skia_safe::Typeface::new(value.typeface(), skia_safe::FontStyle::normal()).unwrap(),
            value.size(),
        );
        font.set_edging(skia_safe::font::Edging::SubpixelAntiAlias);
        font.set_subpixel(true);
        font
    }
}

impl Canvas for SkiaCanvas {
    fn clear(&mut self, color: &Color) {
        self.surface.canvas().clear(color);
    }

    fn save(&mut self) {
        self.surface.canvas().save();
    }

    fn restore(&mut self) {
        self.surface.canvas().restore();
    }

    fn translate(&mut self, point: &geo::Point) {
        self.surface.canvas().translate((point.x, point.y));
    }
    fn scale(&mut self, size: &Size) {
        self.surface.canvas().scale((size.width, size.height));
    }

    fn draw_rect(&mut self, rect: &Rect, paint: &Paint) {
        let rect: skia_safe::Rect = rect.into();
        self.surface.canvas().draw_rect(rect, &paint.into());
    }

    fn draw_rounded_rect(&mut self, rect: &Rect, rx: f32, ry: f32, paint: &Paint) {
        let rect: skia_safe::Rect = rect.into();
        self.surface
            .canvas()
            .draw_round_rect(rect, rx, ry, &paint.into());
    }

    fn draw_circle(&mut self, center: &geo::Point, radius: f32, paint: &Paint) {
        self.surface
            .canvas()
            .draw_circle(*center, radius, &paint.into());
    }

    fn draw_string(&mut self, rect: &Rect, text: &str, font: &Font, paint: &Paint) {
        let rect: skia_safe::Rect = rect.into();
        self.surface.canvas().draw_str_align(
            text,
            rect.center(),
            &font.into(),
            &paint.into(),
            Align::Center,
        );
    }

    fn pixels(&mut self) -> Option<&[u8]> {
        SkiaCanvas::pixels(self)
    }

    // fn draw_text_blob(&mut self, pos: &Point, blob: &skia_safe::TextBlob, paint: &Paint) {
    //     self.surface.canvas().draw_text_blob(blob, *pos, paint);
    // }

    // fn draw_paragraph(&mut self, pos: &Point, paragraph: &skia_safe::textlayout::Paragraph) {
    //     paragraph.paint(self.surface.canvas(), *pos);
    // }
}

unsafe impl Send for SkiaCanvas {}
