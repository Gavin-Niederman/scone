use pollster::FutureExt;
use wgpu::include_wgsl;
use winit::event::Event;
use crate::renderable::Renderable;

use crate::renderer::Renderer;

pub struct State{
    renderer: Renderer,
    window: winit::window::Window,

    test_renderable: Renderable<'static>,
}

impl State {
    pub fn new(window: winit::window::Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window).unwrap() };

        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| adapter.is_surface_supported(&surface))
            .next()
            .unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_texture_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|format| format.describe().srgb)
            .next()
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            alpha_mode: surface_capabilities.alpha_modes[0],
            format: surface_texture_format,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            width: size.width,
            height: size.height,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: vec![],
        };

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    label: None,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .block_on()
            .unwrap();

        surface.configure(&device, &surface_config);

        let shader = device.create_shader_module(include_wgsl!("../resources/shaders/colored.wgsl"));
        
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            depth_stencil: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_texture_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                front_face: wgpu::FrontFace::Ccw,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
                strip_index_format: None,
            },
            multisample: wgpu::MultisampleState { count: 1, mask: !0, alpha_to_coverage_enabled: false },
            multiview: None,
        });

        let renderer = Renderer::new(device, queue, adapter, size, surface, surface_config, render_pipeline);

        let test_renderable = Renderable {
            indicese: &[],
            shader,
        };

        Self { renderer, window, test_renderable }
    }

    pub fn get_window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn handle_event(&mut self, event: &Event<'_, ()>) {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                winit::event::WindowEvent::Resized(new_size) => self.resize(*new_size),
                _ => {}
            },
            _ => {}
        }
        match self.render() {
            Err(wgpu::SurfaceError::Lost) => {
                self.renderer.reconfigure();
            },
            Err(wgpu::SurfaceError::Outdated) => {},
            Err(err) => {
                log::error!("{err}");
            },
            Ok(_) => {}
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render(
            vec![&self.test_renderable]
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use test::Bencher;

    #[bench]
    fn render(bencher: &mut Bencher) {
        use crate::State;
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();
        let mut renderer = State::new(window);
        bencher.iter(|| {
            renderer.render().unwrap();
        });
    }
}
