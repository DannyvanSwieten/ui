use std::collections::HashMap;

use crate::{animation::animation_request::AnimationRequest, tree::ElementId};

#[derive(Default)]
pub struct BuildResult {
    pub animation_requests: HashMap<ElementId, Vec<AnimationRequest>>,
    pub binds: HashMap<ElementId, Vec<String>>,
}
