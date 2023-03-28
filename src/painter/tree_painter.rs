use super::{render_ctx::RenderCtx, PaintCtx, PainterTree};
use crate::{
    animation::{
        animation_ctx::AnimationCtx, animation_event::AnimationEvent,
        animation_request::AnimationRequest,
    },
    canvas::Canvas,
    geo::{Point, Rect, Size},
    tree::ElementId,
};
use std::{
    any::Any,
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

pub struct TreePainter {
    tree: PainterTree,
    rx: Receiver<TreePainterMessage>,
    size: Size,
    dpi: f32,
}

impl TreePainter {
    pub fn new(tree: PainterTree, size: Size, dpi: f32) -> (Self, Sender<TreePainterMessage>) {
        let (tx, rx) = channel();
        let tree_painter = Self {
            size,
            tree,
            rx,
            dpi,
        };
        (tree_painter, tx)
    }

    pub fn call_mounted(&self) -> HashMap<ElementId, AnimationRequest> {
        let mut animation_requests = HashMap::new();
        for (id, node) in self.tree.nodes() {
            if let Some(painter) = node.data().painter() {
                let mut ctx = RenderCtx::new(*id, &mut animation_requests);
                painter.mounted(&mut ctx)
            }
        }

        animation_requests
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

    pub fn set_painter_tree(&mut self, tree: PainterTree) -> HashMap<ElementId, AnimationRequest> {
        self.tree = tree;
        let mut animation_requests = HashMap::new();
        for (id, node) in self.tree.nodes() {
            if let Some(painter) = &node.data.painter {
                let mut render_ctx = RenderCtx::new(*id, &mut animation_requests);
                painter.mounted(&mut render_ctx)
            }
        }

        animation_requests
    }

    pub fn merge_sub_tree(
        &mut self,
        parent: usize,
        tree: PainterTree,
    ) -> HashMap<ElementId, AnimationRequest> {
        self.tree.remove_node(tree.root_id());
        let new_nodes = self.tree.merge_subtree(parent, tree);
        let mut animation_requests = HashMap::new();
        for id in new_nodes {
            if let Some(painter) = &self.tree[id].data.painter {
                let mut render_ctx = RenderCtx::new(id, &mut animation_requests);
                painter.mounted(&mut render_ctx)
            }
        }

        animation_requests
    }

    fn paint_element(&mut self, id: ElementId, offset: Option<Point>, canvas: &mut dyn Canvas) {
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

            if let Some(painter) = node.data.painter() {
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

    pub fn paint(&mut self, offset: Option<Point>, canvas: &mut dyn Canvas) {
        while let Ok(message) = self.rx.try_recv() {
            self.handle_message(message)
        }

        self.paint_element(self.tree.root_id(), offset, canvas)
    }

    pub fn animation(&mut self, animation_events: Vec<(ElementId, AnimationEvent)>) {
        for (id, animation_event) in animation_events {
            if let Some(painter) = &mut self.tree[id].data.painter {
                let mut ctx = AnimationCtx::new(id, animation_event);
                painter.animation_event(&mut ctx)
            }
        }
    }

    fn handle_message(&mut self, message: TreePainterMessage) {
        match message {
            TreePainterMessage::ReplaceTree(tree) => self.tree = tree,
            TreePainterMessage::Resize(size) => self.size = size,
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn dpi(&self) -> f32 {
        self.dpi
    }

    pub fn tree_mut(&mut self) -> &mut PainterTree {
        &mut self.tree
    }
}

pub enum TreePainterMessage {
    ReplaceTree(PainterTree),
    Resize(Size),
}
