use crate::{
    canvas::{
        color::{Color, Color32f},
        skia_cpu_canvas::SkiaCanvas,
        Canvas,
    },
    element_tree::WidgetTree,
    event::{Event, MouseEvent},
    geo::{Point, Rect, Size},
    message_context::MessageCtx,
    std::drag_source::DragSourceData,
    ui_state::UIState,
    widget::Widget,
};

pub struct UserInterface {
    root_tree: WidgetTree,
    width: f32,
    height: f32,
    dpi: f32,
    canvas: Box<dyn Canvas>,
    drag_source: Option<DragSourceData>,
    drag_source_offset: Option<Point>,
    _drag_source_tree: Option<WidgetTree>,
}

impl UserInterface {
    pub fn new(root_widget: Box<dyn Widget>, dpi: f32, width: f32, height: f32) -> Self {
        let width = width;
        let height = height;
        let canvas = Box::new(SkiaCanvas::new(dpi, width as _, height as _));
        let root_tree = WidgetTree::new(root_widget);

        Self {
            root_tree,
            _drag_source_tree: None,
            width,
            height,
            dpi,
            canvas,
            drag_source: None,
            drag_source_offset: None,
        }
    }

    pub fn resize(&mut self, dpi: f32, width: f32, height: f32, state: &UIState) {
        self.width = width;
        self.height = height;
        self.root_tree
            .set_bounds(&Rect::new_from_size(Size::new(width, height)));
        self.canvas = Box::new(SkiaCanvas::new(dpi, width as _, height as _));
        self.layout(state)
    }

    pub fn build(&mut self, state: &mut UIState) {
        self.root_tree.build(state);
        self.layout(state)
    }

    pub fn layout(&mut self, state: &UIState) {
        self.root_tree.layout(state)
    }

    fn paint_drag_source(&mut self, _offset: Option<Point>, _ui_state: &UIState) {
        //todo!()
        // if let Some(data) = self.drag_source.take() {
        //     for item in data.items() {
        //         match item.widget() {
        //             DragSourceWidget::Id(id) => self.paint_element(*id, offset, ui_state),
        //             DragSourceWidget::Widget(_) => {
        //                 println!("Custom widget not implemented yet")
        //             }
        //         }
        //     }

        //     self.drag_source = Some(data)
        // }
    }

    pub fn paint(&mut self, ui_state: &UIState) {
        self.canvas.save();
        self.canvas.scale(&Size::new(self.dpi, self.dpi));
        let c = Color::from(Color32f::new_grey(0.0));
        self.canvas.clear(&c);
        self.root_tree.paint(None, self.canvas.as_mut(), ui_state);
        self.canvas.restore();
        self.paint_drag_source(self.drag_source_offset, ui_state);
    }

    pub fn set_drag_source_position(&mut self, pos: Point) {
        self.drag_source_offset = Some(pos)
    }

    pub fn update_drag_source_position(&mut self, offset: Option<Point>) {
        self.drag_source_offset = offset;
    }

    fn mouse_event(
        &mut self,
        event: &MouseEvent,
        message_ctx: &mut MessageCtx,
        ui_state: &UIState,
    ) {
        let widget_state_updates = self.root_tree.mouse_event(event, message_ctx, ui_state);
        self.root_tree.update_state(&widget_state_updates);
        for (id, _) in widget_state_updates {
            self.root_tree.layout_element(id, ui_state)
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
    }

    pub fn event(&mut self, event: &Event, message_ctx: &mut MessageCtx, ui_state: &UIState) {
        match event {
            Event::Mouse(mouse_event) => {
                self.mouse_event(mouse_event, message_ctx, ui_state);
            }
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
        self.root_tree.handle_mutations(state);

        state.clear_updates()
    }
}
