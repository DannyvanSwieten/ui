use crate::geo::Point;

#[derive(PartialEq, Eq, Hash)]
pub enum MouseEventType {
    MouseDown,
    MouseUp,
    MouseMove,

    DoubleClick,
}

#[derive(Clone, Copy)]
pub struct MouseEvent {
    modifiers: u32,
    global_position: Point,
    local_position: Point,
    delta_position: Point,
    drag_start: Option<Point>,
}

impl MouseEvent {
    pub fn new(modifiers: u32, global_position: &Point, local_position: &Point) -> Self {
        Self {
            modifiers,
            global_position: *global_position,
            local_position: *local_position,
            delta_position: Point::new(0., 0.),
            drag_start: None,
        }
    }

    pub fn to_local(&self, position: &Point) -> Self {
        let mut new_event = *self;
        new_event.local_position = self.local_position - *position;
        new_event
    }

    pub fn new_with_delta(
        modifiers: u32,
        global_position: &Point,
        local_position: &Point,
        delta_position: &Point,
    ) -> Self {
        Self {
            modifiers,
            global_position: *global_position,
            local_position: *local_position,
            delta_position: *delta_position,
            drag_start: None,
        }
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
}
