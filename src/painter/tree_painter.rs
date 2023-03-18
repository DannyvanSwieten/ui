use super::PainterTree;
use crate::{
    canvas::Canvas,
    geo::{Point, Size},
};
use std::sync::mpsc::{channel, Receiver, Sender};

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

    pub fn paint(&mut self, offset: Option<Point>, canvas: &mut dyn Canvas) {
        while let Ok(message) = self.rx.recv() {
            self.handle_message(message)
        }

        self.tree.paint(offset, canvas)
    }

    fn handle_message(&mut self, message: TreePainterMessage) {
        match message {
            TreePainterMessage::ReplaceTree(tree) => self.tree = tree,
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn dpi(&self) -> f32 {
        self.dpi
    }
}

pub enum TreePainterMessage {
    ReplaceTree(PainterTree),
}
