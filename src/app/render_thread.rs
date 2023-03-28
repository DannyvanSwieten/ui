use std::{
    any::Any,
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages};
use winit::window::WindowId;

use crate::{
    animation::{animation_driver::AnimationDriver, animation_event::AnimationEvent, Animation},
    app::LayoutUpdates,
    canvas::{
        canvas_renderer::CanvasRenderer, color::Color32f, skia_cpu_canvas::SkiaCanvas, Canvas,
    },
    geo::{Rect, Size},
    painter::{PainterTree, TreePainter},
    tree::ElementId,
};

pub struct StateUpdate {
    pub window_id: WindowId,
    pub states: HashMap<ElementId, Arc<dyn Any + Send>>,
    pub bounds: HashMap<ElementId, (Rect, Rect)>,
}

pub struct MergeResult {
    pub window_id: WindowId,
    pub parent: Option<ElementId>,
    pub tree: PainterTree,
    pub bounds: HashMap<ElementId, (Rect, Rect)>,
}

unsafe impl Send for StateUpdate {}

pub enum RenderThreadMessage {
    AddWindowPainter((WindowId, TreePainter, CanvasRenderer)),
    WindowSurfaceUpdate(WindowId, f32, Size),
    UpdateBounds(LayoutUpdates),
    StateUpdates(StateUpdate),
    MergeUpdate(MergeResult),
    AddAnimationDriver(WindowId, ElementId, Duration),
}

pub struct RenderSendersAndReceivers {
    pub painter_message_sender: Sender<RenderThreadMessage>,
    pub animation_message_receiver: Receiver<AnimationEvents>,
}

pub struct AnimationEvents {
    pub events: HashMap<WindowId, Vec<(ElementId, AnimationEvent)>>,
}

pub struct Animator {
    time: Instant,
    drivers: HashMap<WindowId, Vec<(ElementId, AnimationDriver)>>,
}

impl Animator {
    pub fn new() -> Self {
        Self {
            time: Instant::now(),
            drivers: HashMap::new(),
        }
    }

    pub fn add_driver(&mut self, window_id: WindowId, element_id: ElementId, duration: Duration) {
        let has_key = self.drivers.contains_key(&window_id);
        if !has_key {
            self.drivers.insert(window_id, Vec::new());
        }

        self.drivers
            .get_mut(&window_id)
            .unwrap()
            .push((element_id, AnimationDriver::new(duration)))
    }

    pub fn tick(&mut self) -> HashMap<WindowId, Vec<(ElementId, AnimationEvent)>> {
        let dt = self.time.elapsed();
        self.time = Instant::now();
        let mut results = HashMap::new();
        for (window_id, drivers) in &mut self.drivers {
            for (element_id, driver) in drivers {
                let animation_event = if driver.value() == 0.0 {
                    AnimationEvent::Start(*element_id)
                } else if driver.value() >= 1.0 {
                    AnimationEvent::End(*element_id)
                } else {
                    AnimationEvent::Update(*element_id, driver.value())
                };

                driver.tick(dt.as_secs_f64());

                let has_key = results.contains_key(window_id);
                if !has_key {
                    results.insert(*window_id, Vec::new());
                }

                results
                    .get_mut(window_id)
                    .unwrap()
                    .push((*element_id, animation_event))
            }
        }

        results
    }
}

impl Default for Animator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RenderThread {
    painters: HashMap<WindowId, TreePainter>,
    canvas: HashMap<WindowId, Box<dyn Canvas>>,
    canvas_renderers: HashMap<WindowId, CanvasRenderer>,
    painter_message_receiver: Receiver<RenderThreadMessage>,
    animation_message_sender: Sender<AnimationEvents>,
    widget_animator: Animator,
    painter_animator: Animator,
}

impl RenderThread {
    pub fn new() -> (Self, RenderSendersAndReceivers) {
        let (painter_message_sender, painter_message_receiver) = channel();
        let (animation_message_sender, animation_message_receiver) = channel();
        let io = RenderSendersAndReceivers {
            painter_message_sender,
            animation_message_receiver,
        };
        (
            Self {
                painters: HashMap::new(),
                canvas: HashMap::new(),
                canvas_renderers: HashMap::new(),
                painter_message_receiver,
                animation_message_sender,
                widget_animator: Animator::new(),
                painter_animator: Animator::new(),
            },
            io,
        )
    }

    fn render(&mut self) {
        for (window_id, painter) in &mut self.painters {
            let canvas = self.canvas.get_mut(window_id).unwrap().as_mut();
            canvas.save();
            canvas.scale(&Size::new(painter.dpi(), painter.dpi()));
            canvas.clear(&Color32f::new_grey(0.4).into());
            painter.paint(None, canvas);
            canvas.restore();
            let renderer = self.canvas_renderers.get_mut(window_id).unwrap();
            let result = renderer.copy_to_texture(canvas);
            if let Ok(texture) = result {
                texture.present()
            }
        }
    }

    pub fn start(mut self) -> JoinHandle<()> {
        thread::spawn(move || loop {
            while let Ok(message) = self.painter_message_receiver.try_recv() {
                match message {
                    RenderThreadMessage::AddWindowPainter((window_id, painter, renderer)) => {
                        let size = *painter.size();
                        let animation_requests = painter.call_mounted();
                        self.painters.insert(window_id, painter);
                        self.canvas.insert(
                            window_id,
                            Box::new(SkiaCanvas::new(size.width as _, size.height as _)),
                        );
                        self.canvas_renderers.insert(window_id, renderer);

                        for (element_id, animation_request) in animation_requests {
                            self.painter_animator.add_driver(
                                window_id,
                                element_id,
                                animation_request.duration,
                            )
                        }
                    }
                    RenderThreadMessage::WindowSurfaceUpdate(window_id, _dpi, size) => {
                        let config = SurfaceConfiguration {
                            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST,
                            alpha_mode: CompositeAlphaMode::Auto,
                            format: TextureFormat::Bgra8Unorm,
                            view_formats: vec![TextureFormat::Bgra8Unorm],
                            width: size.width as _,
                            height: size.height as _,
                            present_mode: PresentMode::Fifo,
                        };
                        let renderer = self.canvas_renderers.get_mut(&window_id).unwrap();
                        renderer.rebuild(config);

                        self.canvas.insert(
                            window_id,
                            Box::new(SkiaCanvas::new(size.width as _, size.height as _)),
                        );
                    }
                    RenderThreadMessage::UpdateBounds(update) => {
                        let painter = self.painters.get_mut(&update.window_id).unwrap();
                        painter.update_bounds(update.bounds)
                    }
                    RenderThreadMessage::StateUpdates(update) => {
                        let painter = self.painters.get_mut(&update.window_id).unwrap();
                        painter.update_bounds(update.bounds);
                        painter.update_state(update.states);
                    }
                    RenderThreadMessage::MergeUpdate(update) => {
                        let painter = self.painters.get_mut(&update.window_id).unwrap();
                        let animation_requests = if let Some(parent) = update.parent {
                            painter.merge_sub_tree(parent, update.tree)
                        } else {
                            painter.set_painter_tree(update.tree)
                        };

                        painter.update_bounds(update.bounds);
                        for (element_id, animation_request) in animation_requests {
                            self.painter_animator.add_driver(
                                update.window_id,
                                element_id,
                                animation_request.duration,
                            )
                        }
                    }
                    RenderThreadMessage::AddAnimationDriver(window_id, element_id, duration) => {
                        self.widget_animator
                            .add_driver(window_id, element_id, duration)
                    }
                }
            }
            let events = self.widget_animator.tick();
            self.animation_message_sender
                .send(AnimationEvents { events })
                .expect("Animation messages send failed");
            let events = self.painter_animator.tick();
            for (window_id, animation_events) in events {
                self.painters
                    .get_mut(&window_id)
                    .unwrap()
                    .animation(animation_events);
            }
            self.render()
        })
    }
}
