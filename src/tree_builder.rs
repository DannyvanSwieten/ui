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

    fn build_element(&mut self, build_ctx: &mut BuildCtx, id: usize) {
        let node = &mut self.tree[id];
        build_ctx.id = id;
        if let Some(state) = node.data.widget().state(build_ctx.ui_state()) {
            node.data.set_state(state)
        }
        for child in node.data.widget().build(build_ctx) {
            let child_id = self.tree.add_node(WidgetElement::new(child));
            self.build_element(build_ctx, child_id);
            self.tree.add_child(id, child_id);
        }
    }

    pub fn build(mut self, ui_state: &mut UIState) -> WidgetTree {
        let mut build_ctx = BuildCtx::new(self.tree.root_id(), ui_state);
        self.build_element(&mut build_ctx, self.tree.root_id());
        self.tree
    }
}
