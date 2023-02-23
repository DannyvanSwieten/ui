use crate::{ui_state::UIState, widget::Widget};

type UIBuilder = dyn Fn(&UIState) -> Box<dyn Widget>;

pub struct WindowRequest {
    pub width: u32,
    pub height: u32,
    pub title: Option<String>,
    builder: Option<Box<UIBuilder>>,
}

impl WindowRequest {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            title: None,
            builder: None,
        }
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn with_ui<F: 'static>(mut self, builder: F) -> Self
    where
        F: Fn(&UIState) -> Box<dyn Widget>,
    {
        self.builder = Some(Box::new(builder));
        self
    }

    pub fn builder(&self) -> &Option<Box<UIBuilder>> {
        &self.builder
    }
}
