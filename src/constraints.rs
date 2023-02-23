#[derive(Default)]
pub struct BoxConstraints {
    min_width: Option<f32>,
    min_height: Option<f32>,
    max_width: Option<f32>,
    max_height: Option<f32>,
}

impl BoxConstraints {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_min(width: f32, height: f32) -> Self {
        Self {
            min_width: Some(width),
            min_height: Some(height),
            ..Default::default()
        }
    }

    pub fn new_with_max(width: f32, height: f32) -> Self {
        Self {
            max_width: Some(width),
            max_height: Some(height),
            ..Default::default()
        }
    }

    pub fn with_min_width(mut self, min_width: f32) -> Self {
        self.min_width = Some(min_width);
        self
    }

    pub fn with_max_width(mut self, max_width: f32) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn with_min_height(mut self, min_height: f32) -> Self {
        self.min_height = Some(min_height);
        self
    }

    pub fn with_max_height(mut self, max_height: f32) -> Self {
        self.max_height = Some(max_height);
        self
    }

    pub fn with_tight_constraints(mut self, width: f32, height: f32) -> Self {
        self.min_width = Some(width);
        self.max_width = Some(width);
        self.min_height = Some(height);
        self.max_height = Some(height);
        self
    }

    pub fn shrunk(&self, dw: f32, dh: f32) -> Self {
        let width = self.max_width.map(|width| width - dw);
        let height = self.max_height.map(|height| height - dh);

        Self {
            min_width: self.min_width,
            min_height: self.min_height,
            max_width: width,
            max_height: height,
        }
    }

    pub fn min_width(&self) -> Option<f32> {
        self.min_width
    }
    pub fn max_width(&self) -> Option<f32> {
        self.max_width
    }

    pub fn min_height(&self) -> Option<f32> {
        self.min_height
    }

    pub fn max_height(&self) -> Option<f32> {
        self.max_height
    }

    pub fn has_min(&self) -> bool {
        self.min_width.is_some() && self.min_height.is_some()
    }

    pub fn has_max(&self) -> bool {
        self.max_width.is_some() && self.max_height.is_some()
    }
}
