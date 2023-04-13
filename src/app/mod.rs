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
    painter::{PainterTreeBuilder, TreePainter},
    tree::ElementId,
    user_interface::{ui_state::UIState, MutationResult, Rebuild, UserInterface},
    widget::message_context::MessageCtx,
    window_request::WindowRequest,
};
use pollster::block_on;
use std::{any::Any, collections::HashMap, sync::Arc, thread::JoinHandle};
use winit::{
    dpi::{LogicalSize, PhysicalPosition},
    event::{DeviceId, ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

use self::{
    event::ApplicationEvent,
    message::Message,
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
    pub messages: Vec<Message>,
    pub update_state: HashMap<ElementId, SetState>,
    pub animation_requests: HashMap<ElementId, Vec<AnimationRequest>>,
}

impl EventResponse {
    pub fn new() -> Self {
        Self {
            window_id: None,
            resize: None,
            messages: Vec::new(),
            update_state: HashMap::new(),
            animation_requests: HashMap::new(),
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
    pub new_states: HashMap<ElementId, Arc<dyn Any + Send>>,
    pub new_bounds: HashMap<ElementId, (Rect, Rect)>,
    pub rebuilds: Vec<Rebuild>,
}

impl EventResolution {
    pub fn new() -> Self {
        Self {
            window_id: None,
            resize: None,
            new_states: HashMap::new(),
            new_bounds: HashMap::new(),
            rebuilds: Vec::new(),
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

pub struct Application {
    ui_state: UIState,
    window_requests: Vec<WindowRequest>,
    pending_messages: Vec<Message>,
    user_interfaces: HashMap<WindowId, UserInterface>,
    windows: HashMap<WindowId, Window>,
    pub io: RenderSendersAndReceivers,
    _render_thread_handle: JoinHandle<()>,
}

impl Application {
    pub fn start(delegate: impl ApplicationDelegate + 'static) {
        let ui_state = delegate.create_ui_state();
        let (render_thread, io) = RenderThread::new();
        let app = Self {
            ui_state,
            window_requests: Vec::new(),
            pending_messages: Vec::new(),
            user_interfaces: HashMap::new(),
            windows: HashMap::new(),
            io,
            _render_thread_handle: render_thread.start(),
        };
        app.run(delegate);
    }

    pub fn handle_focus_change(
        &mut self,
        window_id: &WindowId,
        focused: bool,
        event_response: &mut EventResponse,
    ) {
        if let Some(ui) = self.user_interfaces.get_mut(window_id) {
            let mut message_ctx = MessageCtx::default();
            ui.event(
                &ApplicationEvent::Focus(focused),
                &mut message_ctx,
                &self.ui_state,
                event_response,
            );
        }
    }

    pub fn handle_mouse_input(
        &mut self,
        window_id: &WindowId,
        state: &ElementState,
        button: &MouseButton,
        device_id: &DeviceId,
        event_response: &mut EventResponse,
    ) {
        let mut message_ctx = MessageCtx::default();
        match state {
            ElementState::Pressed => {
                if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                    ui.mouse_down(&mut message_ctx, &self.ui_state, event_response)
                }
            }
            ElementState::Released => {
                if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                    ui.mouse_up(&mut message_ctx, &self.ui_state, event_response);
                    event_response.messages.extend(message_ctx.messages());
                }
            }
        }
    }

    pub fn handle_mouse_cursor_move(
        &mut self,
        window_id: &WindowId,
        position: &PhysicalPosition<f64>,
        event_response: &mut EventResponse,
    ) {
        let mut message_ctx = MessageCtx::default();
        let dpi = self.windows.get(window_id).unwrap().scale_factor();
        let position = position.to_logical::<f32>(dpi);
        let position = Point::new(position.x as _, position.y as _);
        if let Some(ui) = self.user_interfaces.get_mut(window_id) {
            ui.mouse_move(position, &mut message_ctx, &self.ui_state, event_response);
        }
    }

    pub fn handle_window_event(
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
                if let Some(ui) = self.user_interfaces.get_mut(window_id) {
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
            } => todo!(),
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

    pub fn handle_event(
        &mut self,
        delegate: &mut (dyn ApplicationDelegate + 'static),
        event_response: &mut EventResponse,
        control_flow: &mut ControlFlow,
        event: &Event<()>,
    ) {
        match event {
            Event::LoopDestroyed => delegate.app_will_quit(),

            Event::WindowEvent { window_id, event } => {
                self.handle_window_event(window_id, event, delegate, event_response, control_flow)
            }

            Event::MainEventsCleared => {
                self.handle_animation_messages(event_response);
            }

            _ => *control_flow = ControlFlow::Poll,
        }
    }

    fn handle_animation_messages(&mut self, event_response: &mut EventResponse) {
        while let Ok(message) = self.io.animation_message_receiver.try_recv() {
            for (window_id, animation_events) in message.events {
                event_response.set_window_id(window_id);
                for (element_id, event) in animation_events {
                    if let Some(ui) = self.user_interfaces.get_mut(&window_id) {
                        let mut message_ctx = MessageCtx::default();
                        ui.event(
                            &ApplicationEvent::Animation(element_id, event),
                            &mut message_ctx,
                            &self.ui_state,
                            event_response,
                        );
                    }
                }
            }
        }
    }

    fn handle_window_event_resolution(&mut self, event_response: &EventResponse) {
        let window_id = event_response.window_id.unwrap();
        if let Some(ui) = self.user_interfaces.get_mut(&window_id) {
            let resolution = ui.resolve_event_response(event_response, &self.ui_state);

            for rebuild in resolution.rebuilds {
                let painter_tree = PainterTreeBuilder::build(&rebuild.tree, &self.ui_state);
                let parent = rebuild.parent;
                let bounds = ui.merge_rebuild(rebuild, &self.ui_state);
                self.io
                    .painter_message_sender
                    .send(RenderThreadMessage::MergeUpdate(MergeResult {
                        window_id,
                        parent,
                        tree: painter_tree,
                        bounds,
                    }))
                    .expect("Bounds update message send failed");
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

            self.io
                .painter_message_sender
                .send(RenderThreadMessage::StateUpdates(StateUpdate {
                    window_id: event_response.window_id.unwrap(),
                    states: resolution.new_states,
                    bounds: HashMap::new(),
                }))
                .expect("Send failed");
        }
    }

    fn run(mut self, delegate: impl ApplicationDelegate + 'static) {
        let mut delegate = delegate;
        delegate.app_will_start(&mut self);
        self.ui_state = delegate.create_ui_state();
        delegate.app_started(&mut self);
        let event_loop = EventLoop::new();
        let mut painter_trees = HashMap::new();
        let gpu = block_on(GpuApi::new());
        event_loop.run(move |event, event_loop, control_flow| {
            let mut event_response = EventResponse::default();
            self.handle_event(&mut delegate, &mut event_response, control_flow, &event);

            if event_response.window_id.is_some() {
                self.handle_window_event_resolution(&event_response);
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
                    painter_trees.insert(window.id(), message_sender);

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

            for message in event_response.messages {
                self.dispatch(message)
            }

            while let Some(message) = self.pending_messages.pop() {
                delegate.handle_message(message, &mut self.ui_state);
            }

            let mutation_results: Vec<(WindowId, MutationResult)> = self
                .user_interfaces
                .iter_mut()
                .map(|(window_id, ui)| (*window_id, ui.handle_mutations(&mut self.ui_state)))
                .collect();

            for (window_id, result) in mutation_results {
                let ui = self.user_interfaces.get_mut(&window_id).unwrap();
                for rebuild in result.rebuilds {
                    let parent = rebuild.parent;
                    let tree = PainterTreeBuilder::build(&rebuild.tree, &self.ui_state);
                    let bounds = ui.merge_rebuild(rebuild, &self.ui_state);
                    self.io
                        .painter_message_sender
                        .send(RenderThreadMessage::MergeUpdate(MergeResult {
                            window_id,
                            tree,
                            bounds,
                            parent,
                        }))
                        .expect("Merge result send failed");
                }
            }

            self.ui_state.clear_updates();

            // if let Some(state_updates) = event_response.state_updates {
            //     self.io
            //         .painter_message_sender
            //         .send(RenderThreadMessage::StateUpdates(StateUpdate {
            //             window_id: event_response.window_id.unwrap(),
            //             states: state_updates.states,
            //             bounds: HashMap::new(),
            //         }))
            //         .expect("Send failed");
            // }

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

    pub fn request_window(&mut self, request: WindowRequest) {
        self.window_requests.push(request)
    }

    pub fn dispatch(&mut self, message: Message) {
        self.pending_messages.push(message)
    }
}
