use super::AnimationId;

pub enum AnimationEvent {
    Start(AnimationId),
    Update(AnimationId, f64),
    End(AnimationId),
}
