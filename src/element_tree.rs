use std::collections::HashMap;

use crate::element::Element;

pub struct ElementTree {
    elements: HashMap<usize, Element>,
    root_id: usize,
}
