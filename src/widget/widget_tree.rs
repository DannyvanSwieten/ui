use std::collections::HashMap;

use crate::{
    canvas::Canvas,
    constraints::BoxConstraints,
    event::MouseEvent,
    event_context::{EventCtx, SetState},
    geo::{Point, Rect, Size},
    message_context::MessageCtx,
    painter::PaintCtx,
    tree::Tree,
    ui_state::UIState,
    widget::{BuildCtx, LayoutCtx, Widget, WidgetElement},
};

pub struct WidgetTree {
    tree: Tree<WidgetElement>,
}

impl WidgetTree {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        Self {
            tree: Tree::new(WidgetElement::new(widget)),
        }
    }

    pub fn handle_mutations(&mut self, state: &mut UIState) {
        let updates = state.updates().to_vec();
        for id in updates {
            let mut build_ctx = BuildCtx::new(id, state);
            self.rebuild_element(&mut build_ctx, id);
            self.layout_element(id, state)
        }
    }

    pub fn update_state(&mut self, updates: &HashMap<usize, SetState>) {
        for (id, update) in updates {
            let node = self.tree.get_mut(*id).unwrap();
            node.data.set_state(update(node.data.widget_state()));
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
                let mut event_ctx =
                    EventCtx::new(hit, Some(&local_event), node.data.widget_state());
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
                let mut event_ctx =
                    EventCtx::new(intercept, Some(&local_event), node.data.widget_state());
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

    pub fn layout_element(&mut self, id: usize, state: &UIState) {
        let mut layout_ctx = LayoutCtx::new(self);
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
                self.layout_element(child, state)
            }
        }
    }

    fn add_element(&mut self, widget: Box<dyn Widget>) -> usize {
        self.tree.add_node(WidgetElement::new(widget))
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        self.tree.add_child(parent, child)
    }

    pub fn calculate_element_size(
        &self,
        id: usize,
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size> {
        if let Some(node) = self.tree.get(id) {
            node.data
                .widget()
                .calculate_size(&node.children, constraints, layout_ctx)
        } else {
            panic!()
        }
    }

    fn rebuild_element(&mut self, build_ctx: &mut BuildCtx, id: usize) {
        if let Some(node) = self.tree.get(id) {
            node.data.widget().build(build_ctx);
        }
    }

    pub fn layout(&mut self, state: &UIState) {
        self.layout_element(self.tree.root_id(), state);
    }

    pub fn paint(&mut self, offset: Option<Point>, canvas: &mut dyn Canvas, ui_state: &UIState) {
        self.paint_element(self.tree.root_id(), offset, canvas, ui_state)
    }

    fn paint_element(
        &mut self,
        id: usize,
        offset: Option<Point>,
        canvas: &mut dyn Canvas,
        ui_state: &UIState,
    ) {
        let children = if let Some(node) = self.tree.get_mut(id) {
            let global_bounds = node
                .data
                .global_bounds
                .with_offset(offset.unwrap_or(Point::new(0.0, 0.0)));

            let local_bounds = node
                .data
                .local_bounds
                .with_offset(offset.unwrap_or(Point::new(0.0, 0.0)));

            if let Some(painter) = node.data.widget().painter() {
                let paint_ctx =
                    PaintCtx::new(&global_bounds, &local_bounds, node.data.widget_state());
                canvas.save();
                canvas.translate(&local_bounds.position());
                painter.paint(&paint_ctx, ui_state, canvas);
            }
            Some(node.children.clone())
        } else {
            None
        };

        if let Some(children) = children {
            for child in children {
                self.paint_element(child, offset, canvas, ui_state);
            }
        }

        canvas.restore()
    }
}
