use std::{
    any::Any,
    collections::HashMap,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
    thread::{self, JoinHandle},
};

use wgpu::{CompositeAlphaMode, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages};
use winit::window::WindowId;

use crate::{
    app::LayoutUpdates,
    geo::{Rect, Size},
    painter::TreePainter,
};

use super::{
    canvas_renderer::CanvasRenderer, color::Color32f, skia_cpu_canvas::SkiaCanvas, Canvas,
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

pub struct StateUpdate {
    pub window_id: WindowId,
    pub states: HashMap<usize, Arc<dyn Any + Send>>,
    pub bounds: HashMap<usize, (Rect, Rect)>,
}

unsafe impl Send for StateUpdate {}

pub enum PainterManagerMessage {
    AddWindowPainter((WindowId, TreePainter, CanvasRenderer)),
    WindowSurfaceUpdate(WindowId, f32, Size),
    UpdateBounds(LayoutUpdates),
    StateUpdates(StateUpdate),
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
                            Box::new(SkiaCanvas::new(size.width as _, size.height as _)),
                        );
                        self.canvas_renderers.insert(id, renderer);
                    }
                    PainterManagerMessage::WindowSurfaceUpdate(window_id, dpi, size) => {
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
                    PainterManagerMessage::UpdateBounds(update) => {
                        let painter = self.painters.get_mut(&update.window_id).unwrap();
                        painter.tree_mut().update_bounds(update.bounds)
                    }
                    PainterManagerMessage::StateUpdates(update) => {
                        let painter = self.painters.get_mut(&update.window_id).unwrap();
                        painter.tree_mut().update_bounds(update.bounds);
                        painter.tree_mut().update_state(update.states);
                    }
                }
            }

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
        })
    }
}
