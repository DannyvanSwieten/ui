use crate::{
    canvas::Canvas,
    geo::{Point, Rect},
    painter::{PaintCtx, Painter},
    tree::Tree,
    ui_state::UIState,
};
use std::any::Any;

pub struct PainterTree {
    tree: Tree<PainterElement>,
}

impl PainterTree {
    pub fn new(widget: Box<dyn Painter>) -> Self {
        Self {
            tree: Tree::new(PainterElement::new(widget)),
        }
    }

    pub fn element(&self, id: usize) -> Option<&PainterElement> {
        self.tree.get(id).map(|node| &node.data)
    }

    pub fn add_element(&mut self, widget: Box<dyn Painter>) -> usize {
        self.tree.add_node(PainterElement::new(widget))
    }

    pub fn add_child(&mut self, parent: usize, child: usize) {
        self.tree.add_child(parent, child)
    }

    pub fn paint(&mut self, offset: Option<Point>, canvas: &mut dyn Canvas, ui_state: &UIState) {
        self.paint_element(self.tree.root_id(), offset, canvas, ui_state)
    }

    fn paint_element(
        &mut self,
        id: usize,
        offset: Option<Point>,
        canvas: &mut dyn Canvas,
        ui_state: &UIState,
    ) {
        let children = if let Some(node) = self.tree.get_mut(id) {
            let global_bounds = node
                .data
                .global_bounds
                .with_offset(offset.unwrap_or(Point::new(0.0, 0.0)));

            let local_bounds = node
                .data
                .local_bounds
                .with_offset(offset.unwrap_or(Point::new(0.0, 0.0)));

            let paint_ctx = PaintCtx::new(&global_bounds, &local_bounds, node.data.painter_state());
            canvas.save();
            canvas.translate(&local_bounds.position());
            node.data.painter.paint(&paint_ctx, ui_state, canvas);

            Some(node.children.clone())
        } else {
            None
        };

        if let Some(children) = children {
            for child in children {
                self.paint_element(child, offset, canvas, ui_state);
            }
        }

        canvas.restore()
    }
}

pub struct PainterElement {
    painter: Box<dyn Painter>,
    painter_state: Option<Box<dyn Any>>,
    pub local_bounds: Rect,
    pub global_bounds: Rect,
}

impl PainterElement {
    pub fn new(painter: Box<dyn Painter>) -> Self {
        Self {
            painter,
            painter_state: None,
            local_bounds: Rect::default(),
            global_bounds: Rect::default(),
        }
    }

    pub fn painter_state(&self) -> &Option<Box<dyn Any>> {
        &self.painter_state
    }
}
