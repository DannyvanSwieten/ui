pub mod build_result;
pub mod ui_state;
pub mod value;
pub mod widget_tree;
pub mod widget_tree_builder;
use std::{any::Any, collections::HashMap, sync::Arc};

use crate::{
    animation::animation_event::AnimationEvent,
    app::{
        event::{ApplicationEvent, MouseEvent},
        EventResolution, EventResponse,
    },
    event_context::EventCtx,
    geo::{Point, Rect, Size},
    mouse_event::MouseEventData,
    tree::ElementId,
    widget::{
        constraints::BoxConstraints, message_context::MessageCtx, BuildCtx, ChangeResponse,
        LayoutCtx, SizeCtx, Widget,
    },
};

use self::{
    build_result::BuildResult,
    ui_state::UIState,
    widget_tree::{WidgetElement, WidgetTree},
    widget_tree_builder::WidgetTreeBuilder,
};

pub struct UserInterface {
    root_tree: WidgetTree,
    size: Size,
    _drag_source: Option<Box<dyn Any>>,
    mouse_position: Option<Point>,
    mouse_down_elements: Vec<ElementId>,
    dragging: bool,
}

impl UserInterface {
    pub fn new(root_widget: Box<dyn Widget>, size: Size) -> Self {
        Self {
            root_tree: WidgetTree::new(WidgetElement::new(root_widget)),
            size,
            _drag_source: None,
            mouse_down_elements: Vec::new(),
            mouse_position: None,
            dragging: false,
        }
    }

    pub fn set_root_tree(&mut self, tree: WidgetTree) {
        self.root_tree = tree
    }

    pub fn resize(&mut self, size: Size, state: &UIState) -> HashMap<usize, (Rect, Rect)> {
        self.size = size;
        self.root_tree
            .root_mut()
            .set_bounds(&Rect::new_from_size(size));
        self.layout(state)
    }

    fn build_element(&mut self, ui_state: &UIState, id: ElementId, build_result: &mut BuildResult) {
        if let Some(node) = self.root_tree.get_mut(id) {
            if node.data.state().is_none() {
                node.data.set_state(node.data.widget().state(ui_state));
            }

            let widget_state = node.data.state();
            let mut build_ctx = BuildCtx::new(id, widget_state, ui_state);
            let children = node.data.widget().build(&mut build_ctx);
            let animation_requests = build_ctx.animation_requests();
            if !animation_requests.is_empty() {
                build_result
                    .animation_requests
                    .insert(id, build_ctx.animation_requests());
            }

            let binds = build_ctx.binds();
            if !binds.is_empty() {
                build_result.binds.insert(id, binds);
            }
            for child in children {
                let child_id = self.root_tree.add_node(WidgetElement::new(child));
                self.build_element(ui_state, child_id, build_result);
                self.root_tree.add_child(id, child_id);
            }
        } else {
            panic!()
        }
    }

    pub fn build(&mut self, ui_state: &mut UIState) -> (&WidgetTree, BuildResult) {
        let mut build_result = BuildResult::default();
        self.build_element(ui_state, self.root_tree.root_id(), &mut build_result);
        self.layout(ui_state);
        (&self.root_tree, build_result)
    }

    pub fn layout(&mut self, state: &UIState) -> HashMap<usize, (Rect, Rect)> {
        let mut bounds = HashMap::new();
        let root_bounds = self.root_tree[self.root_tree.root_id()].global_bounds;
        bounds.insert(self.root_tree.root_id(), (root_bounds, root_bounds));
        self.layout_element(self.root_tree.root_id(), state, &mut bounds);
        bounds
    }

    pub fn layout_element(
        &mut self,
        id: ElementId,
        state: &UIState,
        results: &mut HashMap<usize, (Rect, Rect)>,
    ) {
        let mut layout_ctx = LayoutCtx::new(id, &self.root_tree, state);
        let children = if let Some(node) = self.root_tree.get(id) {
            node.data.widget().layout(
                state,
                &mut layout_ctx,
                node.local_bounds.size(),
                &node.children,
            );
            Some(node.children.clone())
        } else {
            None
        };

        let child_local_bounds = layout_ctx.bounds();
        let mut child_global_bounds = HashMap::new();
        if let Some(node) = self.root_tree.get(id) {
            for (id, rect) in &child_local_bounds {
                let mut global_bounds = *rect;
                global_bounds.set_position(node.global_bounds.position() + rect.position());
                child_global_bounds.insert(*id, global_bounds);
                results.insert(*id, (global_bounds, *rect));
            }
        }

        for (id, bounds) in &child_local_bounds {
            self.root_tree[*id].local_bounds = *bounds;
        }

        for (id, bounds) in &child_global_bounds {
            self.root_tree[*id].global_bounds = *bounds;
        }

        if let Some(children) = children {
            for child in children {
                self.layout_element(child, state, results)
            }
        }
    }

    pub fn resolve_event_response(
        &mut self,
        response: &EventResponse,
        ui_state: &UIState,
    ) -> EventResolution {
        let mut resolution = EventResolution::default();
        resolution.new_states = self.handle_state_updates(response);
        if !resolution.new_states.is_empty() {
            resolution.new_states.iter().for_each(|(id, new_state)| {
                if let Some(node) = self.root_tree.get_mut(*id) {
                    node.data.set_state(Some(new_state.clone()));
                    let rebuild = self.rebuild_element(*id, ui_state);
                    resolution.rebuilds.push(rebuild);
                }
            });
        }

        if let Some(resize) = &response.resize {
            resolution.new_bounds = self.resize(resize.logical_size(), ui_state)
        }

        resolution
    }

    pub fn handle_state_updates(
        &mut self,
        response: &EventResponse,
    ) -> HashMap<usize, Arc<dyn Any + Send>> {
        let mut results = HashMap::new();
        for (id, modify) in &response.update_state {
            let node = &self.root_tree[*id];
            if let Some(old_state) = node.data.state() {
                results.insert(*id, modify(old_state.as_ref()));
            }
        }

        results
    }

    pub fn process_state_results(
        &mut self,
        ui_state: &UIState,
        results: &HashMap<usize, Arc<dyn Any + Send>>,
    ) -> HashMap<usize, (Rect, Rect)> {
        let mut layout_results = HashMap::new();
        results.iter().for_each(|(id, result)| {
            self.root_tree[*id].data.set_state(Some(result.clone()));

            let mut build_result = BuildResult::default();
            self.build_element(ui_state, *id, &mut build_result);
            self.layout_element(*id, ui_state, &mut layout_results)
        });

        layout_results
    }

    pub fn hit_test(
        &self,
        position: &Point,
        intercepted: &mut Vec<ElementId>,
        hit: &mut Option<ElementId>,
    ) {
        self.hit_test_element(self.root_tree.root_id(), position, intercepted, hit);
    }

    fn hit_test_element(
        &self,
        id: ElementId,
        position: &Point,
        intercepted: &mut Vec<ElementId>,
        hit: &mut Option<ElementId>,
    ) {
        let node = &self.root_tree[id];
        if node.hit_test(position) {
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

    fn send_mouse_event(
        &mut self,
        element_id: ElementId,
        event: &MouseEvent,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
        event_response: &mut EventResponse,
    ) {
        if let Some(node) = &self.root_tree.get(element_id) {
            let local_event = event.to_local(&node.global_bounds.position());
            let state = node.data.state();
            let mut event_ctx =
                EventCtx::new_mouse_event(element_id, true, Some(&local_event), state.as_deref());
            node.data
                .widget()
                .mouse_event(ui_state, &mut event_ctx, message_ctx);

            let consume = event_ctx.consume();
            if let Some(set_state) = consume.set_state {
                event_response.update_state.insert(element_id, set_state);
            }
            // animation requests
            event_response
                .animation_requests
                .insert(element_id, consume.animation_requests);
        }
    }

    pub fn mouse_move(
        &mut self,
        location: Point,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
        event_response: &mut EventResponse,
    ) {
        self.mouse_position = Some(location);
        let event_type = if self.mouse_down_elements.is_empty() {
            MouseEvent::MouseMove(MouseEventData::new(
                0,
                &self.mouse_position.unwrap(),
                &self.mouse_position.unwrap(),
            ))
        } else if !self.dragging {
            self.dragging = true;
            MouseEvent::MouseDragStart(MouseEventData::new(
                0,
                &self.mouse_position.unwrap(),
                &self.mouse_position.unwrap(),
            ))
        } else {
            MouseEvent::MouseDrag(MouseEventData::new(
                0,
                &self.mouse_position.unwrap(),
                &self.mouse_position.unwrap(),
            ))
        };

        let event = ApplicationEvent::Mouse(event_type);
        self.event(&event, message_ctx, ui_state, event_response)
    }

    pub fn mouse_down(
        &mut self,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
        event_response: &mut EventResponse,
    ) {
        let event = ApplicationEvent::Mouse(MouseEvent::MouseDown(MouseEventData::new(
            0,
            &self.mouse_position.unwrap(),
            &self.mouse_position.unwrap(),
        )));
        self.event(&event, message_ctx, ui_state, event_response)
    }

    pub fn mouse_up(
        &mut self,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
        event_response: &mut EventResponse,
    ) {
        if self.dragging {
            self.dragging = false;
            let event = ApplicationEvent::Mouse(MouseEvent::MouseDragEnd(MouseEventData::new(
                0,
                &self.mouse_position.unwrap(),
                &self.mouse_position.unwrap(),
            )));
            self.event(&event, message_ctx, ui_state, event_response)
        }
        let event = ApplicationEvent::Mouse(MouseEvent::MouseUp(MouseEventData::new(
            0,
            &self.mouse_position.unwrap(),
            &self.mouse_position.unwrap(),
        )));
        self.event(&event, message_ctx, ui_state, event_response)
    }

    pub fn mouse_event(
        &mut self,
        event: &MouseEvent,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
        event_response: &mut EventResponse,
    ) {
        let mut intercepted = Vec::new();
        let mut hit = None;
        self.hit_test(event.local_position(), &mut intercepted, &mut hit);
        if let Some(hit) = hit {
            self.send_mouse_event(hit, event, message_ctx, ui_state, event_response)
        }

        for intercept in &intercepted {
            self.send_mouse_event(*intercept, event, message_ctx, ui_state, event_response)
        }

        match event {
            MouseEvent::MouseUp(_) => {
                for mouse_down in self.mouse_down_elements.clone() {
                    self.send_mouse_event(mouse_down, event, message_ctx, ui_state, event_response)
                }

                self.mouse_down_elements.clear();
            }
            MouseEvent::MouseDown(_) => {
                if let Some(hit) = hit {
                    self.mouse_down_elements.push(hit);
                }
                self.mouse_down_elements.extend(intercepted.into_iter())
            }
            MouseEvent::MouseDrag(_) => {
                for mouse_down in self.mouse_down_elements.clone() {
                    self.send_mouse_event(mouse_down, event, message_ctx, ui_state, event_response)
                }
            }
            MouseEvent::MouseDragEnd(_) => {
                for mouse_down in self.mouse_down_elements.clone() {
                    self.send_mouse_event(mouse_down, event, message_ctx, ui_state, event_response)
                }
            }
            _ => {}
        }
    }

    pub fn animation_event(
        &mut self,
        element_id: ElementId,
        event: &AnimationEvent,
        ui_state: &UIState,
        event_response: &mut EventResponse,
    ) {
        let node = &self.root_tree[element_id];

        let state = node.data.state();
        let mut event_ctx =
            EventCtx::new_animation_event(element_id, Some(event), state.as_deref());
        node.data.widget().animation_event(&mut event_ctx, ui_state);
        let consume = event_ctx.consume();
        if let Some(set_state) = consume.set_state {
            event_response.update_state.insert(element_id, set_state);
        }

        event_response
            .animation_requests
            .insert(element_id, consume.animation_requests);
    }

    pub fn event(
        &mut self,
        event: &ApplicationEvent,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
        event_response: &mut EventResponse,
    ) {
        match event {
            ApplicationEvent::Mouse(mouse_event) => {
                self.mouse_event(mouse_event, message_ctx, ui_state, event_response);
            }
            ApplicationEvent::Key(_) => (),
            ApplicationEvent::Resize(_) => (),
            ApplicationEvent::Focus(_) => (),
            ApplicationEvent::Animation(element_id, animation_event) => {
                self.animation_event(*element_id, animation_event, ui_state, event_response)
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.size.width as _
    }

    pub fn height(&self) -> u32 {
        self.size.height as _
    }

    fn notify_state_update(&self, id: ElementId, name: &str) -> Option<ChangeResponse> {
        self.root_tree.nodes()[&id]
            .data()
            .widget()
            .binding_changed(name)
    }

    pub fn handle_mutations(&mut self, ui_state: &mut UIState) -> MutationResult {
        let updates = ui_state.updates();
        let mut actions = HashMap::new();
        for (name, id) in updates {
            actions.insert(*id, self.notify_state_update(*id, name));
        }
        let mut mutation_result = MutationResult::default();
        for (id, action) in actions {
            if let Some(action) = action {
                match action {
                    ChangeResponse::Build => {
                        let rebuild = self.rebuild_element(id, ui_state);
                        mutation_result.rebuilds.push(rebuild)
                    }
                    ChangeResponse::Layout => todo!(),
                    ChangeResponse::Paint => todo!(),
                }
            }
        }

        mutation_result
    }

    /// Removes the node from the tree and from its parent then build a new subtree from the node's widget.
    pub fn rebuild_element(&mut self, id: ElementId, ui_state: &UIState) -> Rebuild {
        let parent = self.root_tree.find_parent(id);
        let mut node = self.root_tree.remove_node(id);
        node.children.clear();
        let tree = WidgetTreeBuilder::new_with_root_node(node, id).build(ui_state);

        Rebuild { parent, id, tree }
    }

    fn merge_subtree(&mut self, parent: usize, tree: WidgetTree) {
        self.root_tree.add_child(parent, tree.root_id());
        for (id, node) in tree.consume_nodes() {
            self.root_tree.add_node_with_id(id, node);
        }
    }

    pub fn merge_rebuild(
        &mut self,
        rebuild: Rebuild,
        ui_state: &UIState,
    ) -> HashMap<usize, (Rect, Rect)> {
        let mut results = HashMap::new();
        if let Some(parent) = rebuild.parent {
            self.merge_subtree(parent, rebuild.tree);
            self.layout_element(parent, ui_state, &mut results)
        } else {
            self.set_root_tree(rebuild.tree);
            results = self.layout(ui_state)
        }

        results
    }

    pub fn calculate_element_size(
        &self,
        id: ElementId,
        constraints: &BoxConstraints,
    ) -> Option<Size> {
        if let Some(node) = self.root_tree.get(id) {
            let size_ctx = SizeCtx::new(id, &self.root_tree);
            node.data
                .widget()
                .calculate_size(&node.children, constraints, &size_ctx)
        } else {
            panic!()
        }
    }
}

pub struct Rebuild {
    pub parent: Option<usize>,
    pub id: ElementId,
    pub tree: WidgetTree,
}

#[derive(Default)]
pub struct MutationResult {
    pub rebuilds: Vec<Rebuild>,
}
