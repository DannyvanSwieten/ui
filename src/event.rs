use crate::geo::Point;

pub enum Event {
    Mouse(MouseEvent),
    Key(KeyEvent),
}

pub enum MouseEvent {
    MouseMove(super::mouse_event::MouseEvent),
    MouseEnter(super::mouse_event::MouseEvent),
    MouseLeave(super::mouse_event::MouseEvent),
    MouseUp(super::mouse_event::MouseEvent),
    MouseDown(super::mouse_event::MouseEvent),
    MouseDrag(super::mouse_event::MouseEvent),
    MouseDragStart(super::mouse_event::MouseEvent),
    MouseDragEnd(super::mouse_event::MouseEvent),
}

impl MouseEvent {
    pub fn local_position(&self) -> &Point {
        match self {
            Self::MouseMove(event) => event.local_position(),
            Self::MouseEnter(event) => event.local_position(),
            Self::MouseLeave(event) => event.local_position(),
            Self::MouseUp(event) => event.local_position(),
            Self::MouseDown(event) => event.local_position(),
            Self::MouseDrag(event) => event.local_position(),
            Self::MouseDragStart(event) => event.local_position(),
            Self::MouseDragEnd(event) => event.local_position(),
        }
    }

    pub fn to_local(&self, position: &Point) -> MouseEvent {
        match self {
            Self::MouseMove(event) => Self::MouseMove(event.to_local(position)),
            Self::MouseEnter(event) => Self::MouseEnter(event.to_local(position)),
            Self::MouseLeave(event) => Self::MouseLeave(event.to_local(position)),
            Self::MouseUp(event) => Self::MouseUp(event.to_local(position)),
            Self::MouseDown(event) => Self::MouseDown(event.to_local(position)),
            Self::MouseDrag(event) => Self::MouseDrag(event.to_local(position)),
            Self::MouseDragStart(event) => Self::MouseDragStart(event.to_local(position)),
            Self::MouseDragEnd(event) => Self::MouseDragEnd(event.to_local(position)),
        }
    }
}

pub enum KeyEvent {
    Input(winit::event::KeyboardInput),
    Char(char),
}
