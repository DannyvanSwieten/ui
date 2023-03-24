use std::{any::Any, collections::HashMap, sync::Arc};

use crate::{
    event::{Event, MouseEvent},
    event_context::SetState,
    geo::{Point, Rect, Size},
    message_context::MessageCtx,
    std::drag_source::DragSourceData,
    ui_state::UIState,
    widget::{ChangeResponse, WidgetTree},
};

pub struct UserInterface {
    root_tree: WidgetTree,
    width: f32,
    height: f32,
    _drag_source: Option<DragSourceData>,
    drag_source_offset: Option<Point>,
    _drag_source_tree: Option<WidgetTree>,
}

impl UserInterface {
    pub fn new(root_tree: WidgetTree, width: f32, height: f32) -> Self {
        let width = width;
        let height = height;

        Self {
            root_tree,
            _drag_source_tree: None,
            width,
            height,
            _drag_source: None,
            drag_source_offset: None,
        }
    }

    pub fn resize(
        &mut self,
        width: f32,
        height: f32,
        state: &UIState,
    ) -> HashMap<usize, (Rect, Rect)> {
        self.width = width;
        self.height = height;
        self.root_tree
            .set_bounds(&Rect::new_from_size(Size::new(width, height)));
        self.layout(state)
    }

    pub fn build(&mut self, state: &mut UIState) -> HashMap<usize, (Rect, Rect)> {
        self.root_tree.build(state);
        self.layout(state)
    }

    pub fn layout(&mut self, state: &UIState) -> HashMap<usize, (Rect, Rect)> {
        self.root_tree.layout(state);
        let mut bounds = HashMap::new();
        for (id, node) in self.root_tree.nodes() {
            bounds.insert(*id, (node.data().global_bounds, node.data().local_bounds));
        }

        bounds
    }

    pub fn set_drag_source_position(&mut self, pos: Point) {
        self.drag_source_offset = Some(pos)
    }

    pub fn update_drag_source_position(&mut self, offset: Option<Point>) {
        self.drag_source_offset = offset;
    }

    pub fn handle_state_updates(
        &mut self,
        state_updates: HashMap<usize, SetState>,
    ) -> HashMap<usize, Arc<dyn Any + Send>> {
        let mut results = HashMap::new();
        for (id, modify) in state_updates {
            let node = self.root_tree.nodes().get(&id).unwrap();
            if let Some(old_state) = node.data.widget_state() {
                results.insert(id, modify(old_state.as_ref()));
            }
        }

        results
    }

    pub fn process_state_results(
        &mut self,
        ui_state: &UIState,
        results: &HashMap<usize, Arc<dyn Any + Send>>,
    ) -> HashMap<usize, (Rect, Rect)> {
        self.root_tree.update_state(results);
        let mut layout_results = HashMap::new();
        results.iter().for_each(|(id, _)| {
            self.root_tree
                .layout_element(*id, ui_state, &mut layout_results)
        });

        layout_results
    }

    fn mouse_event(
        &mut self,
        event: &MouseEvent,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
    ) -> (
        HashMap<usize, Arc<dyn Any + Send>>,
        HashMap<usize, (Rect, Rect)>,
    ) {
        let state_updates = self.root_tree.mouse_event(event, message_ctx, ui_state);
        let new_states = self.handle_state_updates(state_updates);
        let new_bounds = self.process_state_results(ui_state, &new_states);
        (new_states, new_bounds)
        // let new_states = self.root_tree.update_state(&widget_state_updates);
        // let mut layout_results = HashMap::new();
        // for (id, _) in widget_state_updates {
        //     self.root_tree
        //         .layout_element(id, ui_state, &mut layout_results)
        // }

        // // if let MouseEvent::MouseDrag(drag_event) = event {
        // //     if self.drag_source.is_some() {
        // //         self.update_drag_source_position(drag_event.offset_to_drag_start())
        // //     }
        // // }

        // if let MouseEvent::MouseUp(_) = event {
        //     self.drag_source = None;
        //     self.drag_source_offset = None;
        // }
    }

    pub fn event(
        &mut self,
        event: &Event,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
    ) -> (
        HashMap<usize, Arc<dyn Any + Send>>,
        HashMap<usize, (Rect, Rect)>,
    ) {
        match event {
            Event::Mouse(mouse_event) => self.mouse_event(mouse_event, message_ctx, ui_state),
            Event::Key(_) => todo!(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width as _
    }

    pub fn height(&self) -> u32 {
        self.height as _
    }

    pub fn handle_mutations(&mut self, ui_state: &mut UIState) -> MutationResult {
        let actions = self.root_tree.handle_mutations(ui_state);
        let mut mutation_result = MutationResult::default();
        for (id, action) in actions {
            if let Some(action) = action {
                match action {
                    ChangeResponse::Build => {
                        let (parent, subtree) = self.root_tree.rebuild_element(id);
                        if let Some(tree) = subtree {
                            mutation_result.rebuilds.push(Rebuild { parent, id, tree })
                        }
                    }
                    ChangeResponse::Layout => todo!(),
                    ChangeResponse::Paint => todo!(),
                }
            }
        }

        mutation_result
    }
}

pub struct Rebuild {
    pub parent: Option<usize>,
    pub id: usize,
    pub tree: WidgetTree,
}

#[derive(Default)]
pub struct MutationResult {
    pub rebuilds: Vec<Rebuild>,
}
