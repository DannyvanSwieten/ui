use std::{collections::HashMap, time::Duration};

use crate::{animation::animation_request::AnimationRequest, tree::ElementId};

pub struct RenderCtx<'a> {
    id: ElementId,
    animation_requests: &'a mut HashMap<ElementId, AnimationRequest>,
}

impl<'a> RenderCtx<'a> {
    pub fn new(
        id: ElementId,
        animation_requests: &'a mut HashMap<ElementId, AnimationRequest>,
    ) -> Self {
        Self {
            id,
            animation_requests,
        }
    }

    pub fn animation_request(&mut self, duration: Duration) {
        self.animation_requests
            .insert(self.id, AnimationRequest { duration });
    }
}
