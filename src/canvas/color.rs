use std::ops::{Add, Mul};

#[derive(Default, Clone, Copy)]
pub struct Color8u {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color8u {
    pub fn to_u32(&self) -> u32 {
        let r = self.r as u32;
        let g = self.g as u32;
        let g = g << 8;
        let b = self.b as u32;
        let b = b << 16;
        let a = self.a as u32;
        let a = a << 24;
        r | g | b | a
    }

    pub fn as_floats(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }
}

pub type Color = Color8u;

#[derive(Clone, Copy)]
pub struct Color32f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color32f {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn new_grey(g: f32) -> Self {
        Self {
            r: g,
            g,
            b: g,
            a: 1.0,
        }
    }

    pub fn with_alpha(mut self, a: f32) -> Self {
        self.a = a;
        self
    }
}

impl From<Color32f> for Color8u {
    fn from(val: Color32f) -> Self {
        let r = (val.r * 255.0) as u8;
        let g = (val.g * 255.0) as u8;
        let b = (val.b * 255.0) as u8;
        let a = (val.a * 255.0) as u8;
        Color8u { r, g, b, a }
    }
}

impl Mul<f32> for Color32f {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs, self.a * rhs)
    }
}

impl Add<Color32f> for Color32f {
    type Output = Self;
    fn add(self, rhs: Color32f) -> Self::Output {
        Self::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

pub trait Lerp<T: Mul<f32> + Add<T>> {
    fn lerp(&self, rhs: T, x: f32) -> T;
}

impl Lerp<Color32f> for Color32f {
    fn lerp(&self, rhs: Self, x: f32) -> Self {
        *self * (1.0 - x) + rhs * x
    }
}
