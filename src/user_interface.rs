use std::collections::HashMap;

use crate::{
    build_context::BuildCtx, constraints::BoxConstraints, layout_ctx::LayoutCtx, rect::Rect,
    size::Size2D, ui_state::UIState, widget::Widget,
};

pub struct Element {
    widget: Box<dyn Widget>,
    children: Vec<usize>,
    bounds: Rect,
}

impl Element {
    pub fn new<W: Widget + 'static>(widget: W) -> Self {
        Self {
            widget: Box::new(widget),
            children: Vec::new(),
            bounds: Rect::default(),
        }
    }

    pub fn new_box(widget: Box<dyn Widget>) -> Self {
        Self {
            widget,
            children: Vec::new(),
            bounds: Rect::default(),
        }
    }
    pub fn add_child(&mut self, id: usize) {
        self.children.push(id)
    }

    pub fn add_children(&mut self, ids: Vec<usize>) {
        self.children.extend(ids)
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = bounds;
    }

    pub fn children(&self) -> &[usize] {
        &self.children
    }

    pub fn calculate_size(
        &self,
        constraints: &BoxConstraints,
        layout_ctx: &LayoutCtx,
    ) -> Option<(f32, f32)> {
        self.widget
            .calculate_size(&self.children, constraints, layout_ctx)
    }
}

pub struct UserInterface {
    next_id: usize,
    elements: HashMap<usize, Element>,
    root_id: usize,
    width: f32,
    height: f32,
}

impl UserInterface {
    pub fn new(root: Box<dyn Widget>, width: f32, height: f32) -> Self {
        let mut this = Self {
            next_id: 0,
            elements: HashMap::new(),
            root_id: 0,
            width,
            height,
        };

        let root_id = this.add_box_element(root);
        this.elements.get_mut(&root_id).unwrap().bounds =
            Rect::new_from_size(Size2D::new(width, height));
        this.root_id = root_id;
        this
    }

    fn next_id(&mut self) -> usize {
        self.next_id = self.next_id + 1;
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
    ) -> Option<(f32, f32)> {
        if let Some(element) = self.elements.get(&id) {
            element.calculate_size(constraints, layout_ctx)
        } else {
            panic!()
        }
    }

    fn build_element(&mut self, build_ctx: &mut BuildCtx, id: usize) {
        if let Some(element) = self.elements.get_mut(&id) {
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

    pub fn build(&mut self, state: &UIState) {
        let mut build_ctx = BuildCtx::new(self.root_id, state);
        self.build_element(&mut build_ctx, self.root_id);
        self.layout(state)
    }

    pub fn layout(&mut self, state: &UIState) {
        let mut layout_ctx = LayoutCtx::new(self);
        self.layout_element(&mut layout_ctx, self.root_id);
        for (id, bounds) in layout_ctx.bounds() {
            if let Some(element) = self.elements.get_mut(&id) {
                element.set_bounds(bounds);
            }
        }
    }

    pub fn layout_element(&self, layout_ctx: &mut LayoutCtx, id: usize) {
        if let Some(element) = self.elements.get(&id) {
            element
                .widget
                .layout(layout_ctx, element.bounds.size(), &element.children);

            for child in element.children() {
                self.layout_element(layout_ctx, *child)
            }
        }
    }
}
