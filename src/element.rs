use std::any::Any;

use crate::painter::Painter;

pub struct PainterElement {
    painter: Box<dyn Painter>,
    state: Option<Box<dyn Any>>,
}
