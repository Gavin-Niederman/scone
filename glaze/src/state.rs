use pollster::FutureExt;
use winit::event::Event;

use crate::renderer::Renderer;

pub struct State {
    renderer: Renderer,
    window: winit::window::Window,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
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
            present_mode: wgpu::PresentMode::Fifo,
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

        let renderer = Renderer::new(device, queue, adapter);

        Self {
            renderer,
            window,
            size,
            surface,
            surface_config,
        }
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
            Event::RedrawRequested(id) => {
                if self.window.id() == *id {
                    self.render().unwrap_or_else(|err| println!("{err}"));
                }
            }
            _ => {}
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface
                .configure(&self.renderer.device, &self.surface_config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output_texture = self.surface.get_current_texture()?;
        self.renderer.render(output_texture);

        Ok(())
    }
}
