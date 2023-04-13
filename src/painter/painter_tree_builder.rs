use crate::{
    tree::Node,
    user_interface::{ui_state::UIState, widget_tree::WidgetTree},
};

use super::{PainterElement, PainterTree};

pub struct PainterTreeBuilder {}

impl PainterTreeBuilder {
    pub fn build(widget_tree: &WidgetTree, ui_state: &UIState) -> PainterTree {
        let mut painter_tree = PainterTree::default();

        painter_tree.set_root_id(widget_tree.root_id());

        for (id, node) in widget_tree.nodes() {
            let painter = node.data.widget().painter(ui_state);
            let state = node.data.state();
            painter_tree.add_node_with_id(*id, Node::new(PainterElement::new(painter, state)));
            for child in &node.children {
                painter_tree.add_child(*id, *child);
            }
        }

        painter_tree
    }
}
