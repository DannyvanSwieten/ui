use std::time::Duration;

use super::AnimationId;

pub enum AnimationRequest {
    Widget(AnimationId, Duration),
    Painter(AnimationId, Duration),
}
