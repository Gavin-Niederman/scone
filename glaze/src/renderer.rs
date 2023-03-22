use crate::renderable::Renderable;

pub(crate) struct Renderer {
    pub(crate) device: wgpu::Device,
    queue: wgpu::Queue,
    adapter: wgpu::Adapter,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        adapter: wgpu::Adapter,
        size: winit::dpi::PhysicalSize<u32>,
        surface: wgpu::Surface,
        surface_config: wgpu::SurfaceConfiguration,
        render_pipeline: wgpu::RenderPipeline,
    ) -> Self {
        Self {
            device,
            queue,
            adapter,
            size,
            surface,
            surface_config,
            render_pipeline,
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface
                .configure(&self.device, &self.surface_config);
        }
    }

    pub(crate) fn reconfigure(&mut self) {
        self.surface.configure(&self.device, &self.surface_config)
    }

    pub fn render(&mut self, renderables: Vec<&Renderable>) -> Result<(), wgpu::SurfaceError> {
        let output_texture = self.surface.get_current_texture()?;

        let view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        render_pass.set_pipeline(&mut self.render_pipeline);
        render_pass.draw(0..3, 0..1);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output_texture.present();

        Ok(())
    }
}
