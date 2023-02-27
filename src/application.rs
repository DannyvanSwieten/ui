use std::{collections::HashMap, time::Instant};

use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowId},
};

use crate::{
    application_delegate::ApplicationDelegate, canvas::canvas_renderer::CanvasRenderer,
    gpu::GpuApi, message::Message, ui_state::UIState, user_interface::UserInterface,
    window_request::WindowRequest,
};

use pollster::block_on;

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
        let mut user_interfaces: HashMap<WindowId, UserInterface> = HashMap::new();
        let mut canvas_renderers: HashMap<WindowId, CanvasRenderer> = HashMap::new();
        let gpu = block_on(GpuApi::new());
        event_loop.run(move |event, event_loop, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    windows.remove(&window_id);
                    if windows.is_empty() {
                        *control_flow = ControlFlow::Exit
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    window_id,
                } => {
                    if let Some(renderer) = canvas_renderers.get_mut(&window_id) {
                        let config = SurfaceConfiguration {
                            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST,
                            alpha_mode: CompositeAlphaMode::Auto,
                            format: TextureFormat::Bgra8Unorm,
                            view_formats: vec![TextureFormat::Bgra8Unorm],
                            width: size.width as _,
                            height: size.height as _,
                            present_mode: PresentMode::Fifo,
                        };
                        renderer.rebuild(config);
                    }
                    if let Some(ui) = user_interfaces.get_mut(&window_id) {
                        ui.resize(size.width as _, size.height as _)
                    }
                }
                Event::MainEventsCleared => {
                    for (id, ui) in &mut user_interfaces {
                        ui.paint();
                        let width = ui.width();
                        let height = ui.height();
                        if let Some(renderer) = canvas_renderers.get_mut(id) {
                            if let Some(pixels) = ui.pixels() {
                                if let Ok(output) = renderer.copy_to_texture(pixels, width, height)
                                {
                                    output.present()
                                }
                            }
                        }
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { state: s, .. },
                    window_id,
                } => match s {
                    ElementState::Pressed => (),
                    ElementState::Released => (),
                },
                _ => *control_flow = ControlFlow::Poll,
            }

            while let Some(request) = self.window_requests.pop() {
                let window = WindowBuilder::default()
                    .with_active(true)
                    .with_inner_size(LogicalSize::new(
                        request.width as f32,
                        request.height as f32,
                    ))
                    .with_title(request.title.as_ref().unwrap_or(&"Untitled".to_string()))
                    .build(event_loop)
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
                    let surface = unsafe {
                        gpu.instance
                            .create_surface(&window)
                            .expect("Surface Creation Failed")
                    };
                    let config = SurfaceConfiguration {
                        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST,
                        alpha_mode: CompositeAlphaMode::Auto,
                        format: TextureFormat::Bgra8Unorm,
                        view_formats: vec![TextureFormat::Bgra8Unorm],
                        width: request.width,
                        height: request.height,
                        present_mode: PresentMode::Fifo,
                    };
                    surface.configure(&gpu.device, &config);
                    canvas_renderers.insert(
                        window.id(),
                        CanvasRenderer::new(gpu.device.clone(), gpu.queue.clone(), surface),
                    );
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
