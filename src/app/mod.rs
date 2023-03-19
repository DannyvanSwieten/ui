mod application_delegate;

pub use application_delegate::ApplicationDelegate;

use crate::{
    canvas::{canvas_renderer::CanvasRenderer, skia_cpu_canvas::SkiaCanvas, Canvas},
    event::MouseEvent,
    geo::{Point, Size},
    gpu::GpuApi,
    message::Message,
    message_context::MessageCtx,
    mouse_event,
    painter::{PainterTree, TreePainter},
    ui_state::UIState,
    user_interface::UserInterface,
    widget::WidgetTree,
    window_request::WindowRequest,
};
use pollster::block_on;
use std::{
    collections::HashMap,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};
use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

pub struct PainterManager {
    painters: HashMap<WindowId, TreePainter>,
    canvas: HashMap<WindowId, Box<dyn Canvas>>,
    canvas_renderers: HashMap<WindowId, CanvasRenderer>,
    receiver: Receiver<PainterManagerMessage>,
}

pub enum PainterType {
    Pixels,
    Pdf,
    Svg,
}

pub enum PainterManagerMessage {
    AddWindowPainter((WindowId, TreePainter, CanvasRenderer)),
    WindowSurfaceUpdate(WindowId, f32, Size),
}

impl PainterManager {
    pub fn new() -> (Self, Sender<PainterManagerMessage>) {
        let (sender, receiver) = channel();
        (
            Self {
                painters: HashMap::new(),
                canvas: HashMap::new(),
                canvas_renderers: HashMap::new(),
                receiver,
            },
            sender,
        )
    }
    pub fn start(mut self) -> JoinHandle<()> {
        thread::spawn(move || loop {
            while let Ok(message) = self.receiver.try_recv() {
                match message {
                    PainterManagerMessage::AddWindowPainter((id, painter, renderer)) => {
                        let size = *painter.size();
                        let dpi = painter.dpi();
                        self.painters.insert(id, painter);
                        self.canvas.insert(
                            id,
                            Box::new(SkiaCanvas::new(dpi, size.width as _, size.height as _)),
                        );
                        self.canvas_renderers.insert(id, renderer);
                    }
                    PainterManagerMessage::WindowSurfaceUpdate(window_id, dpi, size) => todo!(),
                }
            }

            for (id, painter) in &mut self.painters {
                painter.paint(None, self.canvas.get_mut(id).unwrap().as_mut())
            }
        })
    }
}

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
        let mut state = delegate.create_ui_state();
        delegate.app_started(&mut self);
        let event_loop = EventLoop::new();
        let mut windows: HashMap<WindowId, Window> = HashMap::new();
        let mut user_interfaces: HashMap<WindowId, UserInterface> = HashMap::new();
        let mut canvas_renderers: HashMap<WindowId, CanvasRenderer> = HashMap::new();
        let mut painter_trees = HashMap::new();
        let gpu = block_on(GpuApi::new());
        let mut last_mouse_position = Point::new(0.0, 0.0);
        let mut mouse_down_states = HashMap::new();
        let mut drag_start = None;
        let (painter_manager, painter_sender) = PainterManager::new();
        let _join_handle = painter_manager.start();
        event_loop.run(move |event, event_loop, control_flow| {
            let mut message_ctx = MessageCtx::default();
            match event {
                Event::LoopDestroyed => delegate.app_will_quit(),

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    windows.remove(&window_id);
                    if windows.is_empty() && delegate.quit_when_last_window_closes() {
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
                    let dpi = windows.get(&window_id).unwrap().scale_factor();
                    if let Some(ui) = user_interfaces.get_mut(&window_id) {
                        let size = size.to_logical::<f32>(dpi);
                        ui.resize(dpi as f32, size.width as _, size.height as _, &state)
                    }
                }
                Event::MainEventsCleared => {
                    for (id, ui) in &mut user_interfaces {
                        ui.paint(&state);
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
                    ElementState::Pressed => {
                        if let Some(ui) = user_interfaces.get_mut(&window_id) {
                            let mouse_event = mouse_event::MouseEvent::new(
                                0,
                                &last_mouse_position,
                                &last_mouse_position,
                            );
                            ui.event(
                                &crate::event::Event::Mouse(MouseEvent::MouseDown(mouse_event)),
                                &mut message_ctx,
                                &state,
                            );
                            mouse_down_states.insert(window_id, true);
                        }
                    }
                    ElementState::Released => {
                        if let Some(ui) = user_interfaces.get_mut(&window_id) {
                            let mouse_event = mouse_event::MouseEvent::new(
                                0,
                                &last_mouse_position,
                                &last_mouse_position,
                            );

                            if drag_start.is_some() {
                                ui.event(
                                    &crate::event::Event::Mouse(MouseEvent::MouseDragEnd(
                                        mouse_event,
                                    )),
                                    &mut message_ctx,
                                    &state,
                                );

                                drag_start = None
                            }

                            ui.event(
                                &crate::event::Event::Mouse(MouseEvent::MouseUp(mouse_event)),
                                &mut message_ctx,
                                &state,
                            );
                            mouse_down_states.insert(window_id, false);
                        }
                    }
                },
                Event::WindowEvent {
                    event:
                        WindowEvent::CursorMoved {
                            device_id: _,
                            position,
                            ..
                        },
                    window_id,
                } => {
                    let dpi = windows.get(&window_id).unwrap().scale_factor();
                    let position = position.to_logical::<f32>(dpi);
                    let position = Point::new(position.x as _, position.y as _);
                    if let Some(ui) = user_interfaces.get_mut(&window_id) {
                        let mut mouse_event = mouse_event::MouseEvent::new(0, &position, &position);
                        if let Some(mouse_down) = mouse_down_states.get(&window_id) {
                            if *mouse_down {
                                if drag_start.is_none() {
                                    drag_start = Some(position);
                                    ui.event(
                                        &crate::event::Event::Mouse(MouseEvent::MouseDragStart(
                                            mouse_event,
                                        )),
                                        &mut message_ctx,
                                        &state,
                                    );
                                } else {
                                    mouse_event = mouse_event
                                        .with_delta(
                                            *mouse_event.global_position() - last_mouse_position,
                                        )
                                        .with_drag_start(drag_start);
                                    ui.event(
                                        &crate::event::Event::Mouse(MouseEvent::MouseDrag(
                                            mouse_event,
                                        )),
                                        &mut message_ctx,
                                        &state,
                                    );
                                }
                            }
                        } else {
                            ui.event(
                                &crate::event::Event::Mouse(MouseEvent::MouseMove(mouse_event)),
                                &mut message_ctx,
                                &state,
                            );
                        }
                    }
                    last_mouse_position = Point::new(position.x as _, position.y as _);
                }
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
                    let mut widget_tree = WidgetTree::new(root);
                    widget_tree.build(&mut state);
                    widget_tree.layout(&state);
                    let painter_tree = PainterTree::new(&widget_tree, &self.state);
                    let (tree_painter, message_sender) = TreePainter::new(
                        painter_tree,
                        Size::new(request.width as f32, request.height as f32),
                        window.scale_factor() as f32,
                    );
                    painter_trees.insert(window.id(), message_sender);
                    painter_sender
                        .send(PainterManagerMessage::AddWindowPainter((
                            window.id(),
                            tree_painter,
                            CanvasRenderer::new(&gpu, &window),
                        )))
                        .expect("Send failed");
                    let ui = UserInterface::new(
                        widget_tree,
                        window.scale_factor() as f32,
                        request.width as f32,
                        request.height as f32,
                    );
                    user_interfaces.insert(window.id(), ui);
                    canvas_renderers.insert(window.id(), CanvasRenderer::new(&gpu, &window));
                }

                windows.insert(window.id(), window);
            }

            for message in message_ctx.messages() {
                self.dispatch(message)
            }

            while let Some(message) = self.pending_messages.pop() {
                delegate.handle_message(message, &mut state);
            }

            user_interfaces.iter_mut().for_each(|(_, ui)| {
                ui.handle_mutations(&mut state);
            });
        });
    }

    pub fn request_window(&mut self, request: WindowRequest) {
        self.window_requests.push(request)
    }

    pub fn dispatch(&mut self, message: Message) {
        self.pending_messages.push(message)
    }
}
