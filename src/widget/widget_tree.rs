use crate::{
    constraints::BoxConstraints,
    event::MouseEvent,
    event_context::{EventCtx, SetState},
    geo::{Point, Rect, Size},
    message_context::MessageCtx,
    tree::{Node, Tree},
    ui_state::UIState,
    widget::{BuildCtx, LayoutCtx, Widget},
};
use std::{any::Any, collections::HashMap, sync::Arc};

use super::{layout_ctx::SizeCtx, ChangeResponse};

pub struct WidgetTree {
    tree: Tree<WidgetElement>,
}

impl WidgetTree {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        Self {
            tree: Tree::new(WidgetElement::new(widget)),
        }
    }

    pub fn new_with_root_id(widget: Box<dyn Widget>, root_id: usize) -> Self {
        Self {
            tree: Tree::new_with_root_id(WidgetElement::new(widget), root_id),
        }
    }

    pub fn root_id(&self) -> usize {
        self.tree.root_id()
    }

    pub fn set_root_id(&mut self, id: usize) {
        self.tree.set_root_id(id)
    }

    pub fn nodes(&self) -> &HashMap<usize, Node<WidgetElement>> {
        self.tree.nodes()
    }

    pub fn consume_nodes(self) -> HashMap<usize, Node<WidgetElement>> {
        self.tree.consume_nodes()
    }

    pub fn bounds(&self) -> &Rect {
        &self
            .tree
            .nodes()
            .get(&self.tree.root_id())
            .unwrap()
            .data
            .global_bounds
    }

    fn add_node(&mut self, id: usize, node: Node<WidgetElement>) {
        self.tree.nodes_mut().insert(id, node);
    }

    pub fn merge_subtree(&mut self, parent: usize, tree: Self) {
        self.add_child(parent, tree.root_id());
        for (id, node) in tree.consume_nodes() {
            self.add_node(id, node);
        }
    }

    fn notify_state_update(&self, id: usize, name: &str) -> Option<ChangeResponse> {
        if let Some(node) = self.tree.nodes().get(&id) {
            node.data().widget().binding_changed(name)
        } else {
            None
        }
    }

    pub fn handle_mutations(
        &mut self,
        ui_state: &mut UIState,
    ) -> HashMap<usize, Option<ChangeResponse>> {
        let updates = ui_state.updates();
        let mut actions = HashMap::new();
        for (name, id) in updates {
            actions.insert(*id, self.notify_state_update(*id, name));
            // let mut build_ctx = BuildCtx::new(id, ui_state);
            // self.rebuild_element(&mut build_ctx, id);
            // self.layout_element(id, state)
        }

        actions
    }

    pub fn update_state(&mut self, new_states: &HashMap<usize, Arc<dyn Any + Send>>) {
        for (id, new_state) in new_states {
            let node = self.tree.get_mut(*id).unwrap();
            node.data.set_state(new_state.clone());
        }
    }

    pub fn mouse_event(
        &mut self,
        event: &MouseEvent,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
    ) -> HashMap<usize, SetState> {
        let mut intercepted = Vec::new();
        let mut hit = None;
        self.hit_test(event.local_position(), &mut intercepted, &mut hit);
        let mut widget_states = HashMap::new();
        if let Some(hit) = hit {
            if let Some(node) = self.tree.get_mut(hit) {
                let local_event = event.to_local(&node.data.global_bounds.position());
                let state = node.data.widget_state();
                let mut event_ctx = EventCtx::new(hit, Some(&local_event), state.as_deref());
                node.data
                    .widget()
                    .mouse_event(ui_state, &mut event_ctx, message_ctx);
                if let Some(drag_source) = event_ctx.drag_source() {
                    for _item in drag_source.items() {
                        // item.widget().build(build_ctx);
                        todo!()
                    }
                }

                let set_state = event_ctx.consume_state();
                if let Some(set_state) = set_state {
                    widget_states.insert(hit, set_state);
                }
            }
        }

        for intercept in intercepted {
            if let Some(node) = self.tree.get_mut(intercept) {
                let local_event = event.to_local(&node.data.global_bounds.position());
                let state = node.data.widget_state();
                let mut event_ctx = EventCtx::new(intercept, Some(&local_event), state.as_deref());
                node.data
                    .widget()
                    .mouse_event(ui_state, &mut event_ctx, message_ctx);
                let set_state = event_ctx.consume_state();
                if let Some(set_state) = set_state {
                    widget_states.insert(intercept, set_state);
                }
            }
        }

        widget_states
    }

    pub fn hit_test(
        &self,
        position: &Point,
        intercepted: &mut Vec<usize>,
        hit: &mut Option<usize>,
    ) {
        self.hit_test_element(self.tree.root_id(), position, intercepted, hit);
    }

    fn hit_test_element(
        &self,
        id: usize,
        position: &Point,
        intercepted: &mut Vec<usize>,
        hit: &mut Option<usize>,
    ) {
        if let Some(node) = self.tree.get(id) {
            if node.data.hit_test(position) {
                if node.data.widget().intercept_mouse_events() {
                    intercepted.push(id);
                } else {
                    *hit = Some(id);
                }

                for child in node.children.iter() {
                    self.hit_test_element(*child, position, intercepted, hit)
                }
            }
        }
    }

    pub fn element(&self, id: usize) -> Option<&WidgetElement> {
        self.tree.get(id).map(|node| &node.data)
    }

    pub fn set_bounds(&mut self, bounds: &Rect) {
        let node = self.tree.get_mut(self.tree.root_id()).unwrap();
        node.data.global_bounds = *bounds;
        node.data.local_bounds = *bounds;
    }

    fn build_element(&mut self, build_ctx: &mut BuildCtx, id: usize) {
        if let Some(node) = self.tree.get_mut(id) {
            build_ctx.id = id;
            if let Some(state) = node.data.widget().state(build_ctx.ui_state()) {
                node.data.set_state(state)
            }
            for child in node.data.widget().build(build_ctx) {
                let child_id = self.add_element(child);
                self.build_element(build_ctx, child_id);
                self.add_child(id, child_id);
            }
        } else {
            panic!()
        }
    }

    pub fn build(&mut self, state: &mut UIState) {
        let mut build_ctx = BuildCtx::new(self.tree.root_id(), state);
        self.build_element(&mut build_ctx, self.tree.root_id());
    }

    pub fn layout_element(
        &mut self,
        id: usize,
        state: &UIState,
        results: &mut HashMap<usize, (Rect, Rect)>,
    ) {
        let mut layout_ctx = LayoutCtx::new(id, self, state);
        let children = if let Some(node) = self.tree.get(id) {
            node.data.widget().layout(
                state,
                &mut layout_ctx,
                node.data.local_bounds.size(),
                &node.children,
            );
            Some(node.children.clone())
        } else {
            None
        };

        let child_local_bounds = layout_ctx.bounds();
        let mut child_global_bounds = HashMap::new();
        if let Some(node) = self.tree.get(id) {
            for (id, rect) in &child_local_bounds {
                let mut global_bounds = *rect;
                global_bounds.set_position(node.data.global_bounds.position() + rect.position());
                child_global_bounds.insert(*id, global_bounds);
                results.insert(*id, (global_bounds, *rect));
            }
        }

        for (id, bounds) in &child_local_bounds {
            if let Some(node) = self.tree.get_mut(*id) {
                node.data.local_bounds = *bounds;
            }
        }

        for (id, bounds) in &child_global_bounds {
            if let Some(node) = self.tree.get_mut(*id) {
                node.data.global_bounds = *bounds;
            }
        }

        if let Some(children) = children {
            for child in children {
                self.layout_element(child, state, results)
            }
        }
    }

    fn add_element(&mut self, widget: Box<dyn Widget>) -> usize {
        self.tree.add_node(WidgetElement::new(widget))
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        self.tree.add_child(parent, child)
    }

    fn remove_element(&mut self, id: usize) -> Option<Node<WidgetElement>> {
        self.tree.remove_node(id)
    }

    pub fn calculate_element_size(&self, id: usize, constraints: &BoxConstraints) -> Option<Size> {
        if let Some(node) = self.tree.get(id) {
            let size_ctx = SizeCtx::new(id, self);
            node.data
                .widget()
                .calculate_size(&node.children, constraints, &size_ctx)
        } else {
            panic!()
        }
    }

    /// Removes the node from the tree and from its parent then build a new subtree from the node's widget.
    pub fn rebuild_element(
        &mut self,
        id: usize,
        ui_state: &mut UIState,
    ) -> (Option<usize>, Option<WidgetTree>) {
        let node = self.remove_element(id);
        if let Some(node) = node {
            let widget = node.data.widget;
            let mut tree = WidgetTree::new_with_root_id(widget, id);
            tree.build(ui_state);
            let parent = if let Some(parent) = self.tree.find_parent(id) {
                self.tree.remove_child_from_parent(parent, id);
                Some(parent)
            } else {
                None
            };
            (parent, Some(tree))
        } else {
            (None, None)
        }
    }

    pub fn layout(&mut self, state: &UIState) {
        let mut results = HashMap::new();
        self.layout_element(self.tree.root_id(), state, &mut results);
    }

    pub fn state(&self, id: usize) -> Option<Arc<dyn Any + Send>> {
        if let Some(element) = self.element(id) {
            element.widget_state()
        } else {
            None
        }
    }

    pub fn tree_mut(&mut self) -> &mut Tree<WidgetElement> {
        &mut self.tree
    }
}

pub struct WidgetElement {
    widget: Box<dyn Widget>,
    widget_state: Option<Arc<dyn Any + Send>>,
    pub local_bounds: Rect,
    pub global_bounds: Rect,
}

impl WidgetElement {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        // let widget_state = widget.state();

        Self {
            widget,
            local_bounds: Rect::default(),
            global_bounds: Rect::default(),
            widget_state: None,
        }
    }

    pub fn widget(&self) -> &dyn Widget {
        self.widget.as_ref()
    }

    pub fn widget_state(&self) -> Option<Arc<dyn Any + Send>> {
        self.widget_state.clone()
    }

    pub fn set_state(&mut self, state: Arc<dyn Any + Send>) {
        self.widget_state = Some(state)
    }

    pub fn hit_test(&self, point: &Point) -> bool {
        self.global_bounds.hit_test(point)
    }
}
