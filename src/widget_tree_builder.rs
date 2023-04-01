use crate::{
    tree::ElementId,
    ui_state::UIState,
    widget::{BuildCtx, Widget, WidgetElement, WidgetTree},
};

pub struct WidgetTreeBuilder {
    tree: WidgetTree,
}

impl WidgetTreeBuilder {
    pub fn new(root: Box<dyn Widget>) -> Self {
        Self {
            tree: WidgetTree::new(WidgetElement::new(root)),
        }
    }

    pub fn new_with_root_id(root: Box<dyn Widget>, root_id: ElementId) -> Self {
        Self {
            tree: WidgetTree::new_with_root_id(WidgetElement::new(root), root_id),
        }
    }

    fn build_element(&mut self, ui_state: &mut UIState, id: ElementId) {
        let node = &mut self.tree[id];
        if let Some(state) = node.data.widget().state(ui_state) {
            node.data.set_state(state)
        }

        let mut build_ctx = BuildCtx::new(id, node.data.widget_state(), ui_state);
        for child in node.data.widget().build(&mut build_ctx) {
            let child_id = self.tree.add_node(WidgetElement::new(child));
            self.build_element(ui_state, child_id);
            self.tree.add_child(id, child_id);
        }
    }

    pub fn build(mut self, ui_state: &mut UIState) -> WidgetTree {
        self.build_element(ui_state, self.tree.root_id());
        self.tree
    }
}
