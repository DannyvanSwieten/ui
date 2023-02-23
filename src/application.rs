use std::{collections::HashMap, time::Instant};

use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

use crate::{
    application_delegate::ApplicationDelegate, message::Message, ui_state::UIState,
    user_interface::UserInterface, window_request::WindowRequest,
};

pub struct Application {
    state: UIState,
    window_requests: Vec<WindowRequest>,
    pending_messages: Vec<Message>,
}

impl Application {
    pub fn start(delegate: impl ApplicationDelegate + 'static) {
        let state = delegate.create_ui_state();
        let app = Self {
            state,
            window_requests: Vec::new(),
            pending_messages: Vec::new(),
        };
        app.run(delegate);
    }

    fn run(mut self, mut delegate: impl ApplicationDelegate + 'static) {
        delegate.app_will_start(&mut self);
        let state = delegate.create_ui_state();
        delegate.app_started(&mut self);
        let event_loop = EventLoop::new();
        let mut windows = HashMap::new();
        let mut user_interfaces = HashMap::new();
        event_loop.run(move |_, event_loop, _| {
            while let Some(request) = self.window_requests.pop() {
                let window = WindowBuilder::default()
                    .with_active(true)
                    .with_inner_size(LogicalSize::new(
                        request.width as f32,
                        request.height as f32,
                    ))
                    .with_title(request.title.as_ref().unwrap_or(&"Untitled".to_string()))
                    .build(&event_loop)
                    .expect("Window creation failed");
                if let Some(builder) = request.builder() {
                    let root = (*builder)(&mut self.state);
                    let mut ui =
                        UserInterface::new(root, request.width as f32, request.height as f32);
                    let instant = Instant::now();
                    ui.build(&state);
                    let instant = Instant::now() - instant;
                    println!("UI Full build took: {} milliseconds", instant.as_millis());
                    user_interfaces.insert(window.id(), ui);
                }

                windows.insert(window.id(), window);
            }

            while let Some(message) = self.pending_messages.pop() {
                delegate.handle_message(message, &state);
            }
        });
    }

    pub fn build_ui(&mut self) {
        // let build_ctx = self.ui.build(&self.state);
        // self.state.bind(build_ctx.bindings())
    }

    pub fn request_window(&mut self, request: WindowRequest) {
        self.window_requests.push(request)
    }

    pub fn dispatch(&mut self, message: Message) {
        self.pending_messages.push(message)
    }
}
