use super::PainterTree;
use crate::{canvas::Canvas, geo::Point, ui_state::UIState};
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

    pub fn paint(&mut self, offset: Option<Point>, canvas: &mut dyn Canvas, ui_state: &UIState) {
        while let Ok(message) = self.rx.recv() {
            self.handle_message(message)
        }

        self.tree.paint(offset, canvas, ui_state)
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
