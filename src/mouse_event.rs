use std::{any::Any, rc::Rc};

use winit::window::WindowId;

use crate::geo::Point;

#[derive(PartialEq, Eq, Hash)]
pub enum MouseEventType {
    MouseDown(MouseButton),
    MouseUp(MouseButton),
    MouseMove(MouseButton),

    DoubleClick(MouseButton),
}

#[derive(PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

#[derive(Clone)]
pub struct MouseEventData {
    window_id: WindowId,
    modifiers: u32,
    global_position: Point,
    local_position: Point,
    delta_position: Point,
    drag_start: Option<Point>,
    drag_data: Option<Rc<dyn Any>>,
    scroll: Option<(f32, f32)>,
}

impl MouseEventData {
    pub fn new(
        window_id: WindowId,
        modifiers: u32,
        global_position: &Point,
        local_position: &Point,
    ) -> Self {
        Self {
            window_id,
            modifiers,
            global_position: *global_position,
            local_position: *local_position,
            delta_position: Point::new(0., 0.),
            drag_start: None,
            drag_data: None,
            scroll: None,
        }
    }

    pub fn to_local(&self, position: &Point) -> Self {
        let mut new = self.clone();
        new.local_position = self.local_position - *position;
        new
    }

    pub fn new_with_delta(
        window_id: WindowId,
        modifiers: u32,
        global_position: &Point,
        local_position: &Point,
        delta_position: &Point,
    ) -> Self {
        Self {
            window_id,
            modifiers,
            global_position: *global_position,
            local_position: *local_position,
            delta_position: *delta_position,
            drag_start: None,
            drag_data: None,
            scroll: None,
        }
    }

    pub fn with_scroll(mut self, scroll: (f32, f32)) -> Self {
        self.scroll = Some(scroll);
        self
    }

    pub fn with_delta(mut self, delta: Point) -> Self {
        self.delta_position = delta;
        self
    }

    pub fn with_drag_start(mut self, start: Option<Point>) -> Self {
        self.drag_start = start;
        self
    }

    pub fn offset_to_drag_start(&self) -> Option<Point> {
        self.drag_start
            .as_ref()
            .map(|drag_start| self.global_position - *drag_start)
    }

    pub fn scroll(&self) -> Point {
        Point::new(self.scroll.unwrap().0, self.scroll.unwrap().1)
    }

    pub fn drag_start(&self) -> &Option<Point> {
        &self.drag_start
    }

    pub fn is_control_down(&self) -> bool {
        (self.modifiers & 1) != 0
    }

    pub fn is_shift_down(&self) -> bool {
        (self.modifiers & 2) != 0
    }

    pub fn is_right_mouse(&self) -> bool {
        (self.modifiers & 4) != 0
    }

    pub fn global_position(&self) -> &Point {
        &self.global_position
    }

    pub fn local_position(&self) -> &Point {
        &self.local_position
    }

    pub fn delta_position(&self) -> &Point {
        &self.delta_position
    }

    pub fn with_drag_data(mut self, data: Rc<dyn Any>) -> Self {
        self.drag_data = Some(data);
        self
    }

    pub fn drag_data<T: 'static>(&self) -> Option<&T> {
        self.drag_data
            .as_ref()
            .and_then(|data| data.downcast_ref::<T>())
    }
}
