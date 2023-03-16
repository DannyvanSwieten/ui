use std::{
    any::Any,
    sync::atomic::{AtomicUsize, Ordering},
};

pub static NEXT_ELEMENT_ID: AtomicUsize = AtomicUsize::new(0);
pub fn next_element_id() -> usize {
    NEXT_ELEMENT_ID.fetch_add(1, Ordering::SeqCst) + 1
}

use crate::{
    geo::{Point, Rect},
    painter::Painter,
    widget::Widget,
};

pub struct PainterElement {
    painter: Box<dyn Painter>,
    state: Option<Box<dyn Any>>,
}
