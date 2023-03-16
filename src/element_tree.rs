use std::collections::HashMap;

use crate::{
    canvas::{paint_ctx::PaintCtx, Canvas},
    constraints::BoxConstraints,
    element::{next_element_id, Element},
    event::MouseEvent,
    event_context::{EventCtx, SetState},
    geo::{Point, Rect, Size},
    message_context::MessageCtx,
    ui_state::UIState,
    widget::{BuildCtx, LayoutCtx, Widget},
};

pub struct ElementTree {
    elements: HashMap<usize, Element>,
    root_id: usize,
}

impl ElementTree {
    pub fn new(widget: Box<dyn Widget>) -> Self {
        let mut this = Self {
            elements: HashMap::new(),
            root_id: 0,
        };

        let root_id = this.add_element(widget);
        this.root_id = root_id;
        this
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
            let element = self.elements.get_mut(id).unwrap();
            element.set_state(update(element.widget_state()));
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
            if let Some(element) = self.elements.get_mut(&hit) {
                let local_event = event.to_local(&element.global_bounds().position());
                let mut event_ctx = EventCtx::new(hit, Some(&local_event), element.widget_state());
                element
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
            if let Some(element) = self.elements.get_mut(&intercept) {
                let local_event = event.to_local(&element.global_bounds().position());
                let mut event_ctx =
                    EventCtx::new(intercept, Some(&local_event), element.widget_state());
                element
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
        self.hit_test_element(self.root_id, position, intercepted, hit);
    }

    fn hit_test_element(
        &self,
        id: usize,
        position: &Point,
        intercepted: &mut Vec<usize>,
        hit: &mut Option<usize>,
    ) {
        if let Some(element) = self.elements.get(&id) {
            if element.hit_test(position) {
                if element.widget().intercept_mouse_events() {
                    intercepted.push(id);
                } else {
                    *hit = Some(id);
                }

                for child in element.children() {
                    self.hit_test_element(*child, position, intercepted, hit)
                }
            }
        }
    }

    pub fn element(&self, id: usize) -> Option<&Element> {
        self.elements.get(&id)
    }

    pub fn set_bounds(&mut self, bounds: &Rect) {
        let root_element = self.elements.get_mut(&self.root_id).unwrap();
        root_element.set_global_bounds(bounds);
        root_element.set_local_bounds(bounds);
    }

    fn build_element(&mut self, build_ctx: &mut BuildCtx, id: usize) {
        if let Some(element) = self.elements.get_mut(&id) {
            build_ctx.id = id;
            for child in element.widget().build(build_ctx) {
                let child_id = self.add_element(child);
                self.build_element(build_ctx, child_id);
                self.add_child(id, child_id);
            }
        } else {
            panic!()
        }
    }

    pub fn build(&mut self, state: &mut UIState) {
        let mut build_ctx = BuildCtx::new(self.root_id, state);
        self.build_element(&mut build_ctx, self.root_id);
    }

    pub fn layout_element(&mut self, id: usize, state: &UIState) {
        let mut layout_ctx = LayoutCtx::new(self);
        let children = if let Some(element) = self.elements.get(&id) {
            element.widget().layout(
                state,
                &mut layout_ctx,
                element.local_bounds().size(),
                element.children(),
            );
            Some(element.children_copy())
        } else {
            None
        };

        let child_local_bounds = layout_ctx.bounds();
        let mut child_global_bounds = HashMap::new();
        if let Some(element) = self.elements.get(&id) {
            for (id, rect) in &child_local_bounds {
                let mut global_bounds = *rect;
                global_bounds.set_position(element.global_bounds().position() + rect.position());
                child_global_bounds.insert(*id, global_bounds);
            }
        }

        for (id, bounds) in &child_local_bounds {
            if let Some(element) = self.elements.get_mut(id) {
                element.set_local_bounds(bounds);
            }
        }

        for (id, bounds) in &child_global_bounds {
            if let Some(element) = self.elements.get_mut(id) {
                element.set_global_bounds(bounds);
            }
        }

        if let Some(children) = children {
            for child in children {
                self.layout_element(child, state)
            }
        }
    }

    fn add_element(&mut self, widget: Box<dyn Widget>) -> usize {
        let id = next_element_id();
        self.elements.insert(id, Element::new(widget));
        id
    }

    fn add_child(&mut self, parent: usize, child: usize) {
        if let Some(element) = self.elements.get_mut(&parent) {
            element.add_child(child)
        }
    }

    pub fn calculate_element_size(
        &self,
        id: usize,
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size> {
        if let Some(element) = self.element(id) {
            element.calculate_size(constraints, layout_ctx)
        } else {
            panic!()
        }
    }

    fn rebuild_element(&mut self, build_ctx: &mut BuildCtx, id: usize) {
        let element = self.elements.remove(&id);
        if let Some(element) = element {
            element.widget().build(build_ctx);
            self.elements.insert(id, element);
        }
    }

    pub fn layout(&mut self, state: &UIState) {
        self.layout_element(self.root_id, state);
    }

    pub fn paint(&mut self, offset: Option<Point>, canvas: &mut dyn Canvas, ui_state: &UIState) {
        self.paint_element(self.root_id, offset, canvas, ui_state)
    }

    fn paint_element(
        &mut self,
        id: usize,
        offset: Option<Point>,
        canvas: &mut dyn Canvas,
        ui_state: &UIState,
    ) {
        let children = if let Some(element) = self.elements.get_mut(&id) {
            let mut global_bounds = *element.global_bounds();
            global_bounds = global_bounds.with_offset(offset.unwrap_or(Point::new(0.0, 0.0)));
            let mut local_bounds = *element.local_bounds();
            local_bounds = local_bounds.with_offset(offset.unwrap_or(Point::new(0.0, 0.0)));

            let paint_ctx = PaintCtx::new(&global_bounds, &local_bounds, element.widget_state());
            canvas.save();
            canvas.translate(&local_bounds.position());
            element.widget().paint(&paint_ctx, ui_state, canvas);
            Some(element.children_copy())
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
