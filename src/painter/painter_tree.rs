use crate::{
    canvas::Canvas,
    geo::{Point, Rect},
    painter::{PaintCtx, Painter},
    tree::{Node, Tree},
    ui_state::UIState,
    widget::WidgetTree,
};
use std::{any::Any, collections::HashMap, sync::Arc};

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
            let painter = node.data.widget().painter(ui_state);
            let state = node.data.widget_state();
            this.tree
                .add_node_with_id(*id, PainterElement::new(painter, state));
            for child in &node.children {
                this.tree.add_child(*id, *child);
            }
        }

        this
    }

    pub fn root_id(&self) -> usize {
        self.tree.root_id()
    }

    pub fn update_bounds(&mut self, bounds_map: HashMap<usize, (Rect, Rect)>) {
        let nodes = self.tree.nodes_mut();
        for (id, (global_bounds, local_bounds)) in bounds_map {
            if let Some(node) = nodes.get_mut(&id) {
                node.data.global_bounds = global_bounds;
                node.data.local_bounds = local_bounds;
            }
        }
    }

    pub fn update_state(&mut self, state_map: HashMap<usize, Arc<dyn Any + Send>>) {
        let nodes = self.tree.nodes_mut();
        for (id, new_state) in state_map {
            if let Some(node) = nodes.get_mut(&id) {
                node.data.set_state(Some(new_state))
            }
        }
    }

    pub fn element(&self, id: usize) -> Option<&PainterElement> {
        self.tree.get(id).map(|node| &node.data)
    }

    pub fn add_element(&mut self, id: usize, element: PainterElement) {
        self.tree.add_node_with_id(id, element)
    }

    pub fn add_node_with_id(&mut self, id: usize, node: Node<PainterElement>) {
        self.tree.nodes_mut().insert(id, node);
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

            canvas.save();
            canvas.translate(&local_bounds.position());

            if let Some(painter) = &node.data.painter {
                let paint_ctx =
                    PaintCtx::new(&global_bounds, &local_bounds, node.data.painter_state());
                painter.paint(&paint_ctx, canvas);
            }

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

    pub fn merge_subtree(&mut self, parent: usize, tree: Self) {
        self.add_child(parent, tree.root_id());
        for (id, node) in tree.tree.consume_nodes() {
            self.add_node_with_id(id, node);
        }
    }
}

pub struct PainterElement {
    painter: Option<Box<dyn Painter>>,
    painter_state: Option<Arc<dyn Any + Send>>,
    pub local_bounds: Rect,
    pub global_bounds: Rect,
}
unsafe impl Send for PainterElement {}
impl PainterElement {
    pub fn new(
        painter: Option<Box<dyn Painter>>,
        painter_state: Option<Arc<dyn Any + Send>>,
    ) -> Self {
        Self {
            painter,
            painter_state,
            local_bounds: Rect::default(),
            global_bounds: Rect::default(),
        }
    }

    pub fn with_bounds(mut self, global_bounds: &Rect, local_bounds: &Rect) -> Self {
        self.global_bounds = *global_bounds;
        self.local_bounds = *local_bounds;
        self
    }

    pub fn painter_state(&self) -> Option<&(dyn Any + Send)> {
        self.painter_state.as_deref()
    }

    pub fn set_state(&mut self, state: Option<Arc<dyn Any + Send>>) {
        self.painter_state = state
    }
}
