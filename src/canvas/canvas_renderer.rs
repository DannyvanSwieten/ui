use std::rc::Rc;

use wgpu::{
    CompositeAlphaMode, Device, PresentMode, Queue, Surface, SurfaceConfiguration, SurfaceError,
    SurfaceTexture, TextureFormat, TextureUsages,
};
use winit::window::Window;

use crate::{geo::Size, gpu::GpuApi};

use super::Canvas;

pub struct CanvasRenderer {
    surface: Surface,
    device: Rc<Device>,
    queue: Rc<Queue>,
    size: Size,
}

impl CanvasRenderer {
    pub fn new(gpu: &GpuApi, window: &Window) -> Self {
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
            width: window.inner_size().width as _,
            height: window.inner_size().height as _,
            present_mode: PresentMode::Fifo,
        };
        surface.configure(&gpu.device, &config);

        Self {
            surface,
            device: gpu.device.clone(),
            queue: gpu.queue.clone(),
            size: Size::new(config.width as _, config.height as _),
        }
    }

    pub fn copy_to_texture(&self, canvas: &mut dyn Canvas) -> Result<SurfaceTexture, SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let stride = self.size.width as u32 * 4;
        let texture_size = wgpu::Extent3d {
            width: output.texture.width() as _,
            height: output.texture.height() as _,
            depth_or_array_layers: 1,
        };

        if let Some(pixels) = canvas.pixels() {
            self.queue.write_texture(
                // Tells wgpu where to copy the pixel data
                wgpu::ImageCopyTexture {
                    texture: &output.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                // The actual pixel data
                pixels,
                // The layout of the texture
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(stride),
                    rows_per_image: None,
                },
                texture_size,
            );
        }

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        Ok(output)
    }

    pub fn rebuild(&mut self, config: SurfaceConfiguration) {
        self.surface.configure(&self.device, &config);
        self.size = Size::new(config.width as _, config.height as _)
    }
}

unsafe impl Send for CanvasRenderer {}
