use std::rc::Rc;

use wgpu::{Device, Queue, Surface, SurfaceConfiguration, SurfaceError, SurfaceTexture};

pub struct CanvasRenderer {
    surface: Surface,
    device: Rc<Device>,
    queue: Rc<Queue>,
}

impl CanvasRenderer {
    pub fn new(device: Rc<Device>, queue: Rc<Queue>, surface: Surface) -> Self {
        Self {
            surface,
            device,
            queue,
        }
    }

    pub fn copy_to_texture(
        &self,
        pixels: &[u8],
        width: u32,
        height: u32,
    ) -> Result<SurfaceTexture, SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let stride = width * 4;
        let texture_size = wgpu::Extent3d {
            width: width as _,
            height: height as _,
            depth_or_array_layers: 1,
        };

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
                bytes_per_row: std::num::NonZeroU32::new(stride as _),
                rows_per_image: std::num::NonZeroU32::new(height as _),
            },
            texture_size,
        );

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
        self.surface.configure(&self.device, &config)
    }
}
