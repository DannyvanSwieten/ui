use std::{any::Any, collections::HashMap};

use crate::{
    build_context::BuildCtx,
    canvas::{
        color::{Color, Color32f},
        paint_ctx::PaintCtx,
        skia_cpu_canvas::SkiaCanvas,
        Canvas2D,
    },
    constraints::BoxConstraints,
    event::{Event, MouseEvent},
    event_context::EventCtx,
    layout_ctx::LayoutCtx,
    message_context::MessageCtx,
    point::Point2D,
    rect::Rect,
    size::Size2D,
    ui_state::UIState,
    widget::{
        drag_source::{DragSourceData, DragSourceWidget},
        Widget,
    },
};

pub struct Element {
    widget: Box<dyn Widget>,
    widget_state: Option<Box<dyn Any>>,
    children: Vec<usize>,
    local_bounds: Rect,
    global_bounds: Rect,
}

impl Element {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        let widget_state = widget.state();
        Self {
            widget: Box::new(widget),
            children: Vec::new(),
            local_bounds: Rect::default(),
            global_bounds: Rect::default(),
            widget_state,
        }
    }

    pub fn new_box(widget: Box<dyn Widget>) -> Self {
        let widget_state = widget.state();
        Self {
            widget,
            children: Vec::new(),
            local_bounds: Rect::default(),
            global_bounds: Rect::default(),
            widget_state,
        }
    }
    pub fn add_child(&mut self, id: usize) {
        self.children.push(id)
    }

    pub fn add_children(&mut self, ids: Vec<usize>) {
        self.children.extend(ids)
    }

    pub fn set_local_bounds(&mut self, bounds: &Rect) {
        self.local_bounds = *bounds
    }

    pub fn set_global_bounds(&mut self, bounds: &Rect) {
        self.global_bounds = *bounds
    }

    pub fn children(&self) -> &[usize] {
        &self.children
    }

    pub fn children_copy(&self) -> Vec<usize> {
        self.children.clone()
    }

    pub fn calculate_size(
        &self,
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        self.widget
            .calculate_size(&self.children, constraints, layout_ctx)
    }

    pub fn hit_test(&self, point: &Point2D) -> bool {
        self.global_bounds.hit_test(point)
    }
}

pub struct UserInterface {
    next_id: usize,
    elements: HashMap<usize, Element>,
    root_id: usize,
    width: f32,
    height: f32,
    dpi: f32,
    canvas: Box<dyn Canvas2D>,
    drag_source: Option<DragSourceData>,
    drag_source_offset: Option<Point2D>,
}

impl UserInterface {
    pub fn new(root: Box<dyn Widget>, dpi: f32, width: f32, height: f32) -> Self {
        let canvas = Box::new(SkiaCanvas::new(dpi, width as _, height as _));
        let mut this = Self {
            next_id: 0,
            elements: HashMap::new(),
            root_id: 0,
            width,
            height,
            dpi,
            canvas,
            drag_source: None,
            drag_source_offset: None,
        };

        let root_id = this.add_box_element(root);
        this.elements.get_mut(&root_id).unwrap().local_bounds =
            Rect::new_from_size(Size2D::new(width, height));
        this.elements.get_mut(&root_id).unwrap().global_bounds =
            Rect::new_from_size(Size2D::new(width, height));
        this.root_id = root_id;
        this
    }

    pub fn resize(&mut self, width: f32, height: f32, _state: &UIState) {
        self.width = width;
        self.height = height;
        self.elements
            .get_mut(&self.root_id)
            .unwrap()
            .set_local_bounds(&Rect::new(
                Point2D::new(0.0, 0.0),
                Size2D::new(width, height),
            ));
        self.elements
            .get_mut(&self.root_id)
            .unwrap()
            .set_global_bounds(&Rect::new(
                Point2D::new(0.0, 0.0),
                Size2D::new(width, height),
            ));
        self.canvas = Box::new(SkiaCanvas::new(self.dpi, width as _, height as _));
        self.layout()
    }

    fn next_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    pub fn add_element<W>(&mut self, widget: W) -> usize
    where
        W: Widget + 'static,
    {
        let id = self.next_id();
        self.elements.insert(id, Element::new(widget));
        id
    }

    pub fn add_box_element(&mut self, widget: Box<dyn Widget>) -> usize {
        let id = self.next_id();
        self.elements.insert(id, Element::new_box(widget));
        id
    }

    pub fn add_child(&mut self, parent: usize, child: usize) {
        if let Some(element) = self.elements.get_mut(&parent) {
            element.add_child(child)
        }
    }

    pub fn calculate_element_size(
        &self,
        id: usize,
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<Size2D> {
        if let Some(element) = self.elements.get(&id) {
            element.calculate_size(constraints, layout_ctx)
        } else {
            panic!()
        }
    }

    fn rebuild_element(&mut self, build_ctx: &mut BuildCtx, id: usize) {
        let element = self.elements.remove(&id);
        if let Some(mut element) = element {
            element.widget.build(build_ctx);
            self.elements.insert(id, element);
        }
    }

    fn build_element(&mut self, build_ctx: &mut BuildCtx, id: usize) {
        if let Some(element) = self.elements.get_mut(&id) {
            build_ctx.id = id;
            if let Some(children) = element.widget.build(build_ctx) {
                for child in children.into_iter() {
                    let child_id = self.add_box_element(child);
                    self.build_element(build_ctx, child_id);
                    self.add_child(id, child_id);
                }
            }
        } else {
            panic!()
        }
    }

    pub fn build(&mut self, state: &mut UIState) {
        let mut build_ctx = BuildCtx::new(self.root_id, state);
        self.build_element(&mut build_ctx, self.root_id);
        self.layout();
        // state.bind(build_ctx.bindings())
    }

    pub fn layout(&mut self) {
        self.layout_element(self.root_id);
    }

    pub fn layout_element(&mut self, id: usize) {
        let mut layout_ctx = LayoutCtx::new(self);
        let children = if let Some(element) = self.elements.get(&id) {
            element.widget.layout(
                &mut layout_ctx,
                element.local_bounds.size(),
                &element.children,
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
                global_bounds.set_position(element.global_bounds.position() + rect.position());
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
                self.layout_element(child)
            }
        }
    }

    fn paint_element(&mut self, id: usize, offset: Option<Point2D>) {
        let children = if let Some(element) = self.elements.get_mut(&id) {
            let mut global_bounds = element.global_bounds;
            global_bounds = global_bounds.with_offset(offset.unwrap_or(Point2D::new(0.0, 0.0)));
            let mut local_bounds = element.local_bounds;
            local_bounds = local_bounds.with_offset(offset.unwrap_or(Point2D::new(0.0, 0.0)));

            let paint_ctx = PaintCtx::new(&global_bounds, &local_bounds, &element.widget_state);
            self.canvas.save();
            self.canvas.translate(&local_bounds.position());
            element.widget.paint(&paint_ctx, self.canvas.as_mut());
            Some(element.children_copy())
        } else {
            None
        };

        if let Some(children) = children {
            for child in children {
                self.paint_element(child, offset);
            }
        }

        self.canvas.restore()
    }

    fn paint_drag_source(&mut self, offset: Option<Point2D>) {
        if let Some(data) = self.drag_source.take() {
            for item in data.items() {
                match item.widget() {
                    DragSourceWidget::Id(id) => self.paint_element(*id, offset),
                    DragSourceWidget::Widget(_) => todo!(),
                }
            }

            self.drag_source = Some(data)
        }
    }

    pub fn paint(&mut self) {
        self.canvas.scale(&Size2D::new(self.dpi, self.dpi));
        self.canvas.clear(&Color::from(Color32f::new_grey(0.0)));
        self.paint_element(self.root_id, None);
        self.paint_drag_source(self.drag_source_offset);
    }

    pub fn set_drag_source_position(&mut self, pos: Point2D) {
        self.drag_source_offset = Some(pos)
    }

    pub fn update_drag_source_position(&mut self, offset: Option<Point2D>) {
        self.drag_source_offset = offset;
    }

    fn hit_test(
        &self,
        id: usize,
        position: &Point2D,
        intercepted: &mut Vec<usize>,
        hit: &mut Option<usize>,
    ) {
        if let Some(element) = self.elements.get(&id) {
            if element.hit_test(position) {
                if element.widget.intercept_mouse_events() {
                    intercepted.push(id);
                } else {
                    *hit = Some(id);
                }

                for child in element.children() {
                    self.hit_test(*child, position, intercepted, hit)
                }
            }
        }
    }

    fn mouse_event(&mut self, event: &MouseEvent, message_ctx: &mut MessageCtx) -> Option<usize> {
        let mut intercepted = Vec::new();
        let mut hit = None;
        self.hit_test(
            self.root_id,
            event.local_position(),
            &mut intercepted,
            &mut hit,
        );
        if let Some(hit) = hit {
            if let Some(element) = self.elements.get_mut(&hit) {
                let local_event = event.to_local(&element.global_bounds.position());
                let mut event_ctx = EventCtx::new(hit, Some(&local_event), &element.widget_state);
                element.widget.mouse_event(&mut event_ctx, message_ctx);
                if self.drag_source.is_none() {
                    self.drag_source = event_ctx.drag_source()
                }
                let set_state = event_ctx.consume_state();
                if let Some(mut set_state) = set_state {
                    if let Some(state) = &mut element.widget_state {
                        (set_state)(state.as_mut())
                    }

                    self.layout_element(hit)
                }
            }
        }

        for intercept in intercepted {
            if let Some(element) = self.elements.get_mut(&intercept) {
                let local_event = event.to_local(&element.global_bounds.position());
                let mut event_ctx =
                    EventCtx::new(intercept, Some(&local_event), &element.widget_state);
                element.widget.mouse_event(&mut event_ctx, message_ctx);
                if self.drag_source.is_none() {
                    self.drag_source = event_ctx.drag_source()
                }
                let set_state = event_ctx.consume_state();
                if let Some(mut set_state) = set_state {
                    if let Some(state) = &mut element.widget_state {
                        (set_state)(state.as_mut())
                    }

                    self.layout_element(intercept)
                }
            }
        }

        if let MouseEvent::MouseDrag(drag_event) = event {
            if self.drag_source.is_some() {
                self.update_drag_source_position(drag_event.offset_to_drag_start())
            }
        }

        if let MouseEvent::MouseUp(_) = event {
            self.drag_source = None;
            self.drag_source_offset = None;
        }

        hit
    }

    pub fn event(&mut self, event: &Event, message_ctx: &mut MessageCtx) -> Option<usize> {
        match event {
            Event::Mouse(mouse_event) => self.mouse_event(mouse_event, message_ctx),
            Event::Key(_) => todo!(),
        }
    }

    pub fn pixels(&mut self) -> Option<&[u8]> {
        self.canvas.pixels()
    }

    pub fn width(&self) -> u32 {
        self.width as _
    }

    pub fn height(&self) -> u32 {
        self.height as _
    }

    pub fn handle_mutations(&mut self, state: &mut UIState) {
        let updates = state.updates().to_vec();
        for id in updates {
            let mut build_ctx = BuildCtx::new(id, state);
            self.rebuild_element(&mut build_ctx, id);
            self.layout_element(id)
        }

        state.clear_updates()
    }
}
