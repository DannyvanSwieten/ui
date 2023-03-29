mod application_delegate;

pub use application_delegate::ApplicationDelegate;
pub mod render_thread;
use crate::{
    canvas::canvas_renderer::CanvasRenderer,
    event::MouseEvent,
    geo::{Point, Rect, Size},
    gpu::GpuApi,
    message::Message,
    message_context::MessageCtx,
    mouse_event,
    painter::{PainterTreeBuilder, TreePainter},
    ui_state::UIState,
    user_interface::{MutationResult, UserInterface},
    window_request::WindowRequest,
};
use pollster::block_on;
use std::{any::Any, collections::HashMap, sync::Arc};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

use self::render_thread::{MergeResult, RenderThread, RenderThreadMessage, StateUpdate};

pub struct Resize {
    pub window_id: WindowId,
    pub size: Size,
    pub dpi: f32,
}

pub struct StateUpdates {
    pub window_id: WindowId,
    pub states: HashMap<usize, Arc<dyn Any + Send>>,
}

pub struct LayoutUpdates {
    pub window_id: WindowId,
    pub bounds: HashMap<usize, (Rect, Rect)>,
}

pub struct EventResolution {
    resize: Option<Resize>,
    messages: Vec<Message>,
    state_updates: Option<StateUpdates>,
    layout_updates: Option<LayoutUpdates>,
}

impl EventResolution {
    pub fn new() -> Self {
        Self {
            resize: None,
            messages: Vec::new(),
            state_updates: None,
            layout_updates: None,
        }
    }

    pub fn set_resize(&mut self, resize: Resize) {
        self.resize = Some(resize)
    }

    pub fn set_state_updates(
        &mut self,
        window_id: WindowId,
        states: HashMap<usize, Arc<dyn Any + Send>>,
    ) {
        self.state_updates = Some(StateUpdates { window_id, states })
    }

    pub fn set_layout_updates(
        &mut self,
        window_id: WindowId,
        bounds: HashMap<usize, (Rect, Rect)>,
    ) {
        self.layout_updates = Some(LayoutUpdates { window_id, bounds })
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
    mouse_down_state: HashMap<WindowId, bool>,
    drag_start: Option<Point>,
}

pub struct Application {
    ui_state: UIState,
    window_requests: Vec<WindowRequest>,
    pending_messages: Vec<Message>,
    user_interfaces: HashMap<WindowId, UserInterface>,
    windows: HashMap<WindowId, Window>,
    pub mouse_state: ApplicationMouseState,
}

impl Application {
    pub fn start(delegate: impl ApplicationDelegate + 'static) {
        let ui_state = delegate.create_ui_state();
        let app = Self {
            ui_state,
            window_requests: Vec::new(),
            pending_messages: Vec::new(),
            user_interfaces: HashMap::new(),
            windows: HashMap::new(),
            mouse_state: ApplicationMouseState::default(),
        };
        app.run(delegate);
    }

    pub fn handle_event(
        &mut self,
        delegate: &mut (dyn ApplicationDelegate + 'static),
        control_flow: &mut ControlFlow,
        event: &Event<()>,
    ) -> EventResolution {
        let mut message_ctx = MessageCtx::default();
        let mut resolution = EventResolution::new();
        match event {
            Event::LoopDestroyed => delegate.app_will_quit(),

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                self.windows.remove(window_id);
                if self.windows.is_empty() && delegate.quit_when_last_window_closes() {
                    *control_flow = ControlFlow::Exit
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id,
            } => {
                if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                    let dpi = self.windows.get(window_id).unwrap().scale_factor();
                    let logical_size = size.to_logical::<f32>(dpi);
                    let bounds = ui.resize(
                        logical_size.width as _,
                        logical_size.height as _,
                        &self.ui_state,
                    );
                    resolution.set_layout_updates(*window_id, bounds);
                    resolution.set_resize(Resize {
                        window_id: *window_id,
                        size: Size::new(size.width as _, size.height as _),
                        dpi: dpi as _,
                    })
                }
            }
            Event::MainEventsCleared => {}
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state: s, .. },
                window_id,
            } => match s {
                ElementState::Pressed => {
                    if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                        let mouse_event = mouse_event::MouseEvent::new(
                            0,
                            &self.mouse_state.last_mouse_position,
                            &self.mouse_state.last_mouse_position,
                        );
                        let (state_updates, layout_updates) = ui.event(
                            &crate::event::Event::Mouse(MouseEvent::MouseDown(mouse_event)),
                            &mut message_ctx,
                            &self.ui_state,
                        );

                        resolution.set_state_updates(*window_id, state_updates);
                        resolution.set_layout_updates(*window_id, layout_updates);

                        self.mouse_state.mouse_down_state.insert(*window_id, true);
                    }
                }
                ElementState::Released => {
                    if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                        let mouse_event = mouse_event::MouseEvent::new(
                            0,
                            &self.mouse_state.last_mouse_position,
                            &self.mouse_state.last_mouse_position,
                        );

                        if self.mouse_state.drag_start.is_some() {
                            let (state_updates, layout_updates) = ui.event(
                                &crate::event::Event::Mouse(MouseEvent::MouseDragEnd(mouse_event)),
                                &mut message_ctx,
                                &self.ui_state,
                            );

                            resolution.set_state_updates(*window_id, state_updates);
                            resolution.set_layout_updates(*window_id, layout_updates);

                            self.mouse_state.drag_start = None;
                        }

                        let (state_updates, layout_updates) = ui.event(
                            &crate::event::Event::Mouse(MouseEvent::MouseUp(mouse_event)),
                            &mut message_ctx,
                            &self.ui_state,
                        );

                        resolution.set_state_updates(*window_id, state_updates);
                        resolution.set_layout_updates(*window_id, layout_updates);
                        resolution.messages.extend(message_ctx.messages());
                        self.mouse_state.mouse_down_state.insert(*window_id, false);
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
                let dpi = self.windows.get(window_id).unwrap().scale_factor();
                let position = position.to_logical::<f32>(dpi);
                let position = Point::new(position.x as _, position.y as _);
                if let Some(ui) = self.user_interfaces.get_mut(window_id) {
                    let mut mouse_event = mouse_event::MouseEvent::new(0, &position, &position);
                    if let Some(mouse_down) = self.mouse_state.mouse_down_state.get(window_id) {
                        if *mouse_down {
                            if self.mouse_state.drag_start.is_none() {
                                self.mouse_state.drag_start = Some(position);
                                let (state_updates, layout_updates) = ui.event(
                                    &crate::event::Event::Mouse(MouseEvent::MouseDragStart(
                                        mouse_event,
                                    )),
                                    &mut message_ctx,
                                    &self.ui_state,
                                );

                                resolution.set_state_updates(*window_id, state_updates);
                                resolution.set_layout_updates(*window_id, layout_updates);
                            } else {
                                mouse_event = mouse_event
                                    .with_delta(
                                        *mouse_event.global_position()
                                            - self.mouse_state.last_mouse_position,
                                    )
                                    .with_drag_start(self.mouse_state.drag_start);
                                let (state_updates, layout_updates) = ui.event(
                                    &crate::event::Event::Mouse(MouseEvent::MouseDrag(mouse_event)),
                                    &mut message_ctx,
                                    &self.ui_state,
                                );

                                resolution.set_state_updates(*window_id, state_updates);
                                resolution.set_layout_updates(*window_id, layout_updates);
                            }
                        }
                    } else {
                        let (state_updates, layout_updates) = ui.event(
                            &crate::event::Event::Mouse(MouseEvent::MouseMove(mouse_event)),
                            &mut message_ctx,
                            &self.ui_state,
                        );

                        resolution.set_state_updates(*window_id, state_updates);
                        resolution.set_layout_updates(*window_id, layout_updates);
                    }
                }
                self.mouse_state.last_mouse_position = Point::new(position.x as _, position.y as _);
            }
            _ => *control_flow = ControlFlow::Poll,
        }

        resolution
    }

    fn run(mut self, delegate: impl ApplicationDelegate + 'static) {
        let mut delegate = delegate;
        delegate.app_will_start(&mut self);
        self.ui_state = delegate.create_ui_state();
        delegate.app_started(&mut self);
        let event_loop = EventLoop::new();
        let mut painter_trees = HashMap::new();
        let gpu = block_on(GpuApi::new());
        let (painter_manager, io) = RenderThread::new();
        let _join_handle = painter_manager.start();
        event_loop.run(move |event, event_loop, control_flow| {
            let event_resolution = self.handle_event(&mut delegate, control_flow, &event);

            if let Some(resize) = &event_resolution.resize {
                io.painter_message_sender
                    .send(RenderThreadMessage::WindowSurfaceUpdate(
                        resize.window_id,
                        resize.dpi,
                        Size::new(resize.size.width, resize.size.height),
                    ))
                    .expect("Painter message send failed");

                if let Some(layout_updates) = event_resolution.layout_updates {
                    io.painter_message_sender
                        .send(RenderThreadMessage::UpdateBounds(layout_updates))
                        .expect("Bounds update message send failed")
                }
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
                    let mut ui =
                        UserInterface::new(root, request.width as f32, request.height as f32);
                    let widget_tree = ui.build(&mut self.ui_state);
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

                    io.painter_message_sender
                        .send(RenderThreadMessage::AddWindowPainter((
                            window.id(),
                            tree_painter,
                            CanvasRenderer::new(&gpu, &window),
                        )))
                        .expect("Send failed");
                }

                self.windows.insert(window.id(), window);
            }

            for message in event_resolution.messages {
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
                    io.painter_message_sender
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

            if let Some(state_updates) = event_resolution.state_updates {
                io.painter_message_sender
                    .send(RenderThreadMessage::StateUpdates(StateUpdate {
                        window_id: state_updates.window_id,
                        states: state_updates.states,
                        bounds: HashMap::new(),
                    }))
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
