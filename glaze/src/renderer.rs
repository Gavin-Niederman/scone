pub struct Renderer {
    pub(crate) device: wgpu::Device,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter,
}

impl Renderer {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, adapter: wgpu::Adapter) -> Self {
        Self {
            device,
            queue,
            adapter,
        }
    }

    pub fn render(&mut self, texture: wgpu::SurfaceTexture) {
        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.5,
                        g: 0.5,
                        b: 0.5,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        texture.present();
    }
}
