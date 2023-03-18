use super::PainterTree;
use crate::{canvas::Canvas, geo::Point};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct TreePainter {
    tree: PainterTree,
    rx: Receiver<TreePainterMessage>,
}

impl TreePainter {
    pub fn new(tree: PainterTree) -> (Self, Sender<TreePainterMessage>) {
        let (tx, rx) = channel();
        let tree_painter = Self { tree, rx };
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
}

pub enum TreePainterMessage {
    ReplaceTree(PainterTree),
}
