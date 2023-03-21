use pollster::FutureExt;
use winit::event::Event;

use crate::renderer::Renderer;

pub struct State {
    renderer: Renderer,
    window: winit::window::Window,
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
            present_mode: wgpu::PresentMode::AutoVsync,
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

        let renderer = Renderer::new(device, queue, adapter, size, surface, surface_config);

        Self { renderer, window }
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
        // log::warn!("redrawing");
        self.render().unwrap_or_else(|err| println!("{err}"));
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.renderer.render()
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
