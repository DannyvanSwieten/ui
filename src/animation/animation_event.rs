use crate::tree::ElementId;

pub enum AnimationEvent {
    Start(ElementId),
    Update(ElementId, f64),
    End(ElementId),
}
