use std::time::Duration;

use super::Animation;

// Maps a duration to 0.0 - 1.0 based on the current progress
pub struct AnimationDriver {
    progress: f64,
    duration: Duration,
}

impl AnimationDriver {
    pub fn new(duration: Duration) -> Self {
        Self {
            progress: 0.0,
            duration,
        }
    }
}

impl Animation<f64> for AnimationDriver {
    fn tick(&mut self, dt: f64) {
        self.progress += dt
    }

    fn value(&self) -> f64 {
        self.progress / self.duration.as_secs_f64()
    }
}
