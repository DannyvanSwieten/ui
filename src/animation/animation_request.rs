use std::time::Duration;

use super::AnimationId;
#[derive(Clone, Copy)]
pub enum AnimationRequest {
    Widget(AnimationId, Duration),
    Painter(AnimationId, Duration),
}
