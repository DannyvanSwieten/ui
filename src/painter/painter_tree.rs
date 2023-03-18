use crate::{
    canvas::Canvas,
    geo::{Point, Rect, Size},
    painter::{PaintCtx, Painter},
    tree::Tree,
    ui_state::UIState,
    widget::WidgetTree,
};
use std::any::Any;

pub struct PainterTree {
    tree: Tree<PainterElement>,
}

impl PainterTree {
    pub fn new(widget_tree: &WidgetTree, ui_state: &UIState) -> Self {
        let mut this = Self {
            tree: Tree::default(),
        };

        this.tree.set_root_id(widget_tree.root_id());

        for (id, node) in widget_tree.nodes() {
            if let Some(painter) = node.data.widget().painter(ui_state) {
                this.tree
                    .add_node_with_id(*id, PainterElement::new(painter));
                for child in &node.children {
                    this.tree.add_child(*id, *child);
                }
            }
        }

        this
    }

    pub fn element(&self, id: usize) -> Option<&PainterElement> {
        self.tree.get(id).map(|node| &node.data)
    }

    pub fn add_element(&mut self, id: usize, element: PainterElement) {
        self.tree.add_node_with_id(id, element)
    }

    pub fn add_child(&mut self, parent: usize, child: usize) {
        self.tree.add_child(parent, child)
    }

    pub fn paint(&mut self, offset: Option<Point>, canvas: &mut dyn Canvas) {
        self.paint_element(self.tree.root_id(), offset, canvas)
    }

    fn paint_element(&mut self, id: usize, offset: Option<Point>, canvas: &mut dyn Canvas) {
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
            node.data.painter.paint(&paint_ctx, canvas);

            Some(node.children.clone())
        } else {
            None
        };

        if let Some(children) = children {
            for child in children {
                self.paint_element(child, offset, canvas);
            }
        }

        canvas.restore()
    }
}

pub struct PainterElement {
    painter: Box<dyn Painter>,
    painter_state: Option<Box<dyn Any + Send>>,
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

    pub fn with_bounds(mut self, global_bounds: &Rect, local_bounds: &Rect) -> Self {
        self.global_bounds = *global_bounds;
        self.local_bounds = *local_bounds;
        self
    }

    pub fn painter_state(&self) -> &Option<Box<dyn Any + Send>> {
        &self.painter_state
    }
}
