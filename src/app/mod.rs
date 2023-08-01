mod application_delegate;
pub mod event;

pub use application_delegate::ApplicationDelegate;
pub mod message;
pub mod render_thread;
use crate::{
    animation::animation_request::AnimationRequest,
    canvas::canvas_renderer::CanvasRenderer,
    event_context::SetState,
    geo::{Point, Rect, Size},
    gpu::GpuApi,
    painter::{tree_painter::TreePainterMessage, PainterTreeBuilder, TreePainter},
    tree::ElementId,
    user_interface::{ui_state::UIState, widget_tree::WidgetTree, Rebuild, UserInterface},
    widget::{message_context::ApplicationCtx, Widget},
    window_request::WindowRequest,
};
use pollster::block_on;
use std::{
    any::Any,
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::JoinHandle,
};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{DeviceId, ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder, WindowId},
};

use self::{
    event::ApplicationEvent,
    message::ApplicationMessage,
    render_thread::{
        MergeResult, RenderSendersAndReceivers, RenderThread, RenderThreadMessage, StateUpdate,
    },
};

pub struct Resize {
    pub window_id: WindowId,
    pub size: Size,
    pub dpi: f32,
}

impl Resize {
    pub fn logical_size(&self) -> Size {
        Size::new(self.size.width / self.dpi, self.size.height / self.dpi)
    }
}

pub struct StateUpdates {
    pub states: HashMap<usize, Arc<dyn Any + Send>>,
}

pub struct LayoutUpdates {
    pub window_id: WindowId,
    pub bounds: HashMap<ElementId, (Rect, Rect)>,
}

pub struct EventResponse {
    pub window_id: Option<WindowId>,
    pub resize: Option<Resize>,
    pub animation_requests: HashMap<ElementId, Vec<AnimationRequest>>,
    pub drag_widget: Option<Box<dyn Widget>>,
}

impl EventResponse {
    pub fn new() -> Self {
        Self {
            window_id: None,
            resize: None,
            animation_requests: HashMap::new(),
            drag_widget: None,
        }
    }

    pub fn set_window_id(&mut self, window_id: WindowId) {
        self.window_id = Some(window_id)
    }

    pub fn set_resize(&mut self, resize: Resize) {
        self.resize = Some(resize)
    }

    pub fn add_animation_requests(
        &mut self,
        element_id: ElementId,
        requests: Vec<AnimationRequest>,
    ) {
        self.animation_requests.insert(element_id, requests);
    }
}

impl Default for EventResponse {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EventResolution {
    pub window_id: Option<WindowId>,
    pub resize: Option<Resize>,
    pub new_bounds: HashMap<ElementId, (Rect, Rect)>,
    pub drag_widget_tree: Option<WidgetTree>,
}

impl EventResolution {
    pub fn new() -> Self {
        Self {
            window_id: None,
            resize: None,
            new_bounds: HashMap::new(),
            drag_widget_tree: None,
        }
    }

    pub fn set_window_id(&mut self, window_id: WindowId) {
        self.window_id = Some(window_id)
    }
}

impl Default for EventResolution {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct ApplicationMouseState {
    last_mouse_position: Point,
    drag_start: Option<Point>,
}

#[derive(Clone)]
pub struct Senders {
    application_message_sender: Sender<ApplicationMessage>,
    state_update_sender: Sender<(ElementId, SetState)>,
}

impl Senders {
    pub fn application_message_queue(&self) -> &Sender<ApplicationMessage> {
        &self.application_message_sender
    }

    pub fn state_update_queue(&self) -> &Sender<(ElementId, SetState)> {
        &self.state_update_sender
    }
}

pub struct Receivers {
    application_message_receiver: Receiver<ApplicationMessage>,
    state_update_receiver: Receiver<(ElementId, SetState)>,
}

pub struct Application {
    ui_state: UIState,
    window_requests: Vec<WindowRequest>,
    senders: Senders,
    receivers: Receivers,
    user_interfaces: HashMap<WindowId, UserInterface>,
    painter_trees: HashMap<WindowId, Sender<TreePainterMessage>>,
    windows: HashMap<WindowId, Window>,
    pub io: RenderSendersAndReceivers,
    _render_thread_handle: JoinHandle<()>,
}

impl Application {
    pub fn start(delegate: impl ApplicationDelegate + 'static) {
        let ui_state = delegate.create_ui_state();
        let (render_thread, io) = RenderThread::new();
        let (application_message_sender, application_message_receiver) = channel();
        let (state_update_sender, state_update_receiver) = channel();
        let app = Self {
            ui_state,
            window_requests: Vec::new(),
            receivers: Receivers {
                application_message_receiver,
                state_update_receiver,
            },
            senders: Senders {
                application_message_sender,
                state_update_sender,
            },
            painter_trees: HashMap::new(),
            user_interfaces: HashMap::new(),
            windows: HashMap::new(),
            io,
            _render_thread_handle: render_thread.start(),
        };
        app.run(delegate);
    }

    fn senders(&self) -> Senders {
        self.senders.clone()
    }

    fn handle_focus_change(
        &mut self,
        window_id: &WindowId,
        focused: bool,
        event_response: &mut EventResponse,
    ) {
        if let Some(ui) = self.user_interfaces.get_mut(window_id) {
            let mut message_ctx = ApplicationCtx::new(self.senders.clone());
            ui.application_event(
                &ApplicationEvent::Focus(focused),
                &mut message_ctx,
                &self.ui_state,
                event_response,
                self.senders.clone(),
            );
        }
    }

    fn handle_mouse_input(
        &mut self,
        window_id: &WindowId,
        state: &ElementState,
        _button: &MouseButton,
        _device_id: &DeviceId,
        event_response: &mut EventResponse,
    ) {
        let mut message_ctx = ApplicationCtx::new(self.senders.clone());
        match state {
            ElementState::Pressed => {
                if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                    ui.mouse_down(
                        *window_id,
                        &mut message_ctx,
                        &self.ui_state,
                        event_response,
                        self.senders.clone(),
                    )
                }
            }
            ElementState::Released => {
                if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                    ui.mouse_up(
                        *window_id,
                        &mut message_ctx,
                        &self.ui_state,
                        event_response,
                        self.senders.clone(),
                    );
                }
            }
        }
    }

    fn handle_mouse_cursor_move(
        &mut self,
        window_id: &WindowId,
        position: &PhysicalPosition<f64>,
        event_response: &mut EventResponse,
    ) {
        let mut message_ctx = ApplicationCtx::new(self.senders.clone());
        let dpi = self.windows.get(window_id).unwrap().scale_factor();
        let position = position.to_logical::<f32>(dpi);
        let position = Point::new(position.x as _, position.y as _);
        if let Some(ui) = self.user_interfaces.get_mut(window_id) {
            ui.mouse_move(
                *window_id,
                position,
                &mut message_ctx,
                &self.ui_state,
                event_response,
                self.senders.clone(),
            );
        }
    }

    fn handle_mouse_scroll(
        &mut self,
        window_id: &WindowId,
        delta: &MouseScrollDelta,
        event_response: &mut EventResponse,
    ) {
        let scroll = match delta {
            MouseScrollDelta::LineDelta(x, y) => (*x, *y),
            MouseScrollDelta::PixelDelta(_) => todo!(),
        };

        let mut message_ctx = ApplicationCtx::new(self.senders.clone());
        if let Some(ui) = self.user_interfaces.get_mut(window_id) {
            ui.mouse_scroll(
                *window_id,
                scroll,
                &mut message_ctx,
                &self.ui_state,
                event_response,
                self.senders.clone(),
            )
        }
    }

    fn handle_window_event(
        &mut self,
        window_id: &WindowId,
        event: &WindowEvent,
        delegate: &mut dyn ApplicationDelegate,
        event_response: &mut EventResponse,
        control_flow: &mut ControlFlow,
    ) {
        event_response.set_window_id(*window_id);
        match event {
            WindowEvent::Resized(size) => {
                if let Some(_ui) = self.user_interfaces.get_mut(window_id) {
                    let dpi = self.windows.get(window_id).unwrap().scale_factor();
                    event_response.set_resize(Resize {
                        window_id: *window_id,
                        size: Size::new(size.width as _, size.height as _),
                        dpi: dpi as _,
                    });
                }
            }
            WindowEvent::Moved(_) => (), //todo!("Moved"),
            WindowEvent::CloseRequested => {
                self.windows.remove(window_id);
                if self.windows.is_empty() && delegate.quit_when_last_window_closes() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            WindowEvent::Destroyed => {
                self.user_interfaces.remove(window_id);
            }
            WindowEvent::DroppedFile(_) => todo!(),
            WindowEvent::HoveredFile(_) => todo!(),
            WindowEvent::HoveredFileCancelled => todo!(),
            WindowEvent::ReceivedCharacter(_) => todo!(),
            WindowEvent::Focused(state) => {
                self.handle_focus_change(window_id, *state, event_response);
            }
            WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => (), //todo!(),
            WindowEvent::ModifiersChanged(_) => (), //todo!(),
            WindowEvent::Ime(_) => todo!(),
            WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => self.handle_mouse_cursor_move(window_id, position, event_response),
            WindowEvent::CursorEntered { device_id } => (),
            WindowEvent::CursorLeft { device_id } => (),
            WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                ..
            } => self.handle_mouse_scroll(window_id, delta, event_response),
            WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => {
                self.handle_mouse_input(window_id, state, button, device_id, event_response);
            }
            WindowEvent::TouchpadMagnify {
                device_id,
                delta,
                phase,
            } => todo!(),
            WindowEvent::SmartMagnify { device_id } => todo!(),
            WindowEvent::TouchpadRotate {
                device_id,
                delta,
                phase,
            } => todo!(),
            WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => todo!(),
            WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => todo!(),
            WindowEvent::Touch(_) => todo!(),
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => todo!(),
            WindowEvent::ThemeChanged(_) => todo!(),
            WindowEvent::Occluded(_) => (), // todo!("Occluded"),
        }
    }

    fn ui_rebuild(&mut self, window_id: &WindowId, rebuild: Rebuild) {
        let painter_tree = PainterTreeBuilder::build(&rebuild.tree, &self.ui_state);
        let parent = rebuild.parent;
        let bounds = self
            .user_interfaces
            .get_mut(window_id)
            .unwrap()
            .merge_rebuild(rebuild, &self.ui_state);
        self.io
            .painter_message_sender
            .send(RenderThreadMessage::MergeUpdate(MergeResult {
                window_id: *window_id,
                parent,
                tree: painter_tree,
                bounds,
            }))
            .expect("Bounds update message send failed");
    }

    fn process_window_event(&mut self, window_id: &WindowId) {
        while let Ok(message) = self.receivers.state_update_receiver.try_recv() {
            if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                let rebuild = ui.set_state(message.0, message.1, &self.ui_state);
                self.ui_rebuild(window_id, rebuild)
            }
        }
    }

    fn handle_event(
        &mut self,
        delegate: &mut (dyn ApplicationDelegate + 'static),
        event_response: &mut EventResponse,
        control_flow: &mut ControlFlow,
        event: &Event<()>,
    ) {
        match event {
            Event::LoopDestroyed => delegate.app_will_quit(),

            Event::WindowEvent { window_id, event } => {
                self.handle_window_event(window_id, event, delegate, event_response, control_flow);
                self.process_window_event(window_id);
            }

            Event::MainEventsCleared => {
                self.handle_animation_messages(event_response);
            }

            _ => *control_flow = ControlFlow::Poll,
        }
    }

    fn handle_animation_messages(&mut self, event_response: &mut EventResponse) {
        let queue = self.senders.clone();
        while let Ok(message) = self.io.animation_message_receiver.try_recv() {
            for (window_id, animation_events) in message.events {
                event_response.set_window_id(window_id);
                for (element_id, event) in animation_events {
                    if let Some(ui) = self.user_interfaces.get_mut(&window_id) {
                        let mut message_ctx = ApplicationCtx::new(queue.clone());
                        ui.application_event(
                            &ApplicationEvent::Animation(element_id, event),
                            &mut message_ctx,
                            &self.ui_state,
                            event_response,
                            self.senders.clone(),
                        );
                    }
                }
            }
        }
    }

    fn handle_window_event_resolution(&mut self, event_response: &mut EventResponse) {
        let window_id = event_response.window_id.unwrap();
        if let Some(ui) = self.user_interfaces.get_mut(&window_id) {
            let resolution = ui.resolve_event_response(event_response, &self.ui_state);

            if let Some(drag_tree) = resolution.drag_widget_tree {
                let painter_tree = PainterTreeBuilder::build(&drag_tree, &self.ui_state);
                self.io
                    .painter_message_sender
                    .send(RenderThreadMessage::DragWidgetCreated(
                        window_id,
                        painter_tree,
                    ))
                    .expect("Drag update message send failed");
            }

            if let Some(resize) = &event_response.resize {
                self.io
                    .painter_message_sender
                    .send(RenderThreadMessage::WindowSurfaceUpdate(
                        resize.window_id,
                        resize.dpi,
                        Size::new(resize.size.width, resize.size.height),
                    ))
                    .expect("Painter message send failed");

                self.io
                    .painter_message_sender
                    .send(RenderThreadMessage::UpdateBounds(LayoutUpdates {
                        window_id,
                        bounds: resolution.new_bounds,
                    }))
                    .expect("Bounds update message send failed")
            }
        }
    }

    fn run(mut self, delegate: impl ApplicationDelegate + 'static) {
        let mut delegate = delegate;
        delegate.app_will_start(&mut self);
        self.ui_state = delegate.create_ui_state();
        delegate.app_started(&mut self);
        let event_loop = EventLoop::new();
        let gpu = block_on(GpuApi::new());
        event_loop.run(move |event, event_loop, control_flow| {
            self.handle_window_requests(&gpu, &event_loop);

            let mut event_response = EventResponse::default();
            self.handle_event(&mut delegate, &mut event_response, control_flow, &event);

            if event_response.window_id.is_some() {
                self.handle_window_event_resolution(&mut event_response);
            }

            while let Ok(message) = self.receivers.application_message_receiver.try_recv() {
                delegate.handle_message(message, &mut self.ui_state);
            }

            let mutation_results: Vec<(WindowId, EventResponse)> = self
                .user_interfaces
                .iter_mut()
                .map(|(window_id, ui)| {
                    (
                        *window_id,
                        ui.handle_mutations(&mut self.ui_state, self.senders.clone()),
                    )
                })
                .collect();

            for (window_id, mut event_response) in mutation_results {
                self.process_window_event(&window_id);
                let ui = self.user_interfaces.get_mut(&window_id).unwrap();
                ui.resolve_event_response(&mut event_response, &self.ui_state);
            }

            self.ui_state.clear_updates();

            for (element_id, animation_requests) in event_response.animation_requests {
                self.io
                    .painter_message_sender
                    .send(RenderThreadMessage::AnimationRequest(
                        event_response.window_id.unwrap(),
                        element_id,
                        animation_requests,
                    ))
                    .expect("Send failed");
            }
        });
    }

    fn handle_window_requests(&mut self, gpu: &GpuApi, event_loop: &EventLoopWindowTarget<()>) {
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
                let root = (*builder)(&mut self.ui_state);
                let mut ui = UserInterface::new(
                    root,
                    Size::new(request.width as f32, request.height as f32),
                );
                let (widget_tree, build_result) = ui.build(&mut self.ui_state);
                for (element_id, bindings) in build_result.binds {
                    for bind in bindings {
                        self.ui_state.bind_one(element_id, &bind);
                    }
                }
                let painter_tree = PainterTreeBuilder::build(widget_tree, &self.ui_state);
                self.user_interfaces.insert(window.id(), ui);
                let (tree_painter, message_sender) = TreePainter::new(
                    painter_tree,
                    Size::new(
                        window.inner_size().width as f32,
                        window.inner_size().height as f32,
                    ),
                    window.scale_factor() as f32,
                );
                self.painter_trees.insert(window.id(), message_sender);

                self.io
                    .painter_message_sender
                    .send(RenderThreadMessage::AddWindowPainter((
                        window.id(),
                        tree_painter,
                        CanvasRenderer::new(&gpu, &window),
                    )))
                    .expect("Send failed");

                for (element_id, animation_requests) in build_result.animation_requests {
                    self.io
                        .painter_message_sender
                        .send(RenderThreadMessage::AnimationRequest(
                            window.id(),
                            element_id,
                            animation_requests,
                        ))
                        .expect("Send failed");
                }
            }

            self.windows.insert(window.id(), window);
        }
    }

    pub fn request_window(&mut self, request: WindowRequest) {
        self.window_requests.push(request)
    }

    pub fn send(&mut self, message: ApplicationMessage) {
        self.senders
            .application_message_queue()
            .send(message)
            .expect("Application Message Send Failed");
    }
}
