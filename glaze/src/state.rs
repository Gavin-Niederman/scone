use pollster::FutureExt;
use winit::window::Window;

use crate::{Error, InitError, MiscError};

pub struct State {
    window: Window,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl State {
    pub fn new(window: Window) -> Result<Self, Error> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .find(|adapter| adapter.is_surface_supported(&surface))
            .ok_or(Error::Init(InitError::NoSupportedAdapter))?;

        log::debug!("{:?}", adapter);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .block_on()
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|capability| capability.describe().srgb)
            .ok_or(Error::Init(InitError::SurfaceLacksCapabilities))?;

        if surface_caps.present_modes.is_empty() {
            return Err(Error::Init(InitError::SurfaceIncompatible));
        }

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        Ok(Self {
            window,
            size,
            surface,
            device,
            queue,
            config,
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) -> Result<(), Error> {
        if new_size.height == 0 || new_size.width == 0 {
            return Err(Error::Misc(MiscError::WindowTooSmall));
        }
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        Ok(())
    }

    pub fn reconfigure(&self) {
        self.surface.configure(&self.device, &self.config)
    }

    pub fn update(&self) {
        //todo
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output_texture = self.surface.get_current_texture()?;
        let view = output_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });

        let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: true,
                }
            })],
            depth_stencil_attachment: None,
        });
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output_texture.present();

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
