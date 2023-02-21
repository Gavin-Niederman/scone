use bytemuck::cast_slice;
use pollster::FutureExt;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::{
    vertex::{ColoredVertex3D, Vertex},
    Error, InitError, MiscError, pipeline, renderable::Rendereable,
};

pub struct State {
    window: Window,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipelines: Vec<wgpu::RenderPipeline>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    renderable_queue: Vec<Box<dyn Rendereable<Vertex = dyn Vertex>>>
}

const TEST_VERTS: &[ColoredVertex3D] = &[
    ColoredVertex3D {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    ColoredVertex3D {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    ColoredVertex3D {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    ColoredVertex3D {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    ColoredVertex3D {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    },
];

const TEST_INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub struct VertexColorRenderableTest<'a> {
    verts: &'a [ColoredVertex3D],
    indices: &'a [u16],
}
impl<'a> Rendereable for VertexColorRenderableTest<'a> {
    type Vertex = ColoredVertex3D;

    fn get_vertices(&self) -> &[Self::Vertex] {
        self.verts
    }
    fn get_indices(&self) -> &[u32] {
        cast_slice(self.indices)
    }
    fn get_shadertype() -> crate::shader::ShaderType {
        crate::shader::ShaderType::VertexColor
    }
}

const VERTEX_COLOR_TEST: VertexColorRenderableTest = VertexColorRenderableTest {
    verts: &[
        ColoredVertex3D {
            position: [-0.0868241, 0.49240386, 0.0],
            color: [0.5, 0.0, 0.5],
        },
        ColoredVertex3D {
            position: [-0.49513406, 0.06958647, 0.0],
            color: [0.5, 0.0, 0.5],
        },
        ColoredVertex3D {
            position: [-0.21918549, -0.44939706, 0.0],
            color: [0.5, 0.0, 0.5],
        },
        ColoredVertex3D {
            position: [0.35966998, -0.3473291, 0.0],
            color: [0.5, 0.0, 0.5],
        },
        ColoredVertex3D {
            position: [0.44147372, 0.2347359, 0.0],
            color: [0.5, 0.0, 0.5],
        },
    ],
    indices: &[0, 1, 4, 1, 2, 4, 2, 3, 4],
};

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

        let shader = device.create_shader_module(wgpu::include_wgsl!(
            "../resources/shaders/vertex_color.wgsl"
        ));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(TEST_VERTS),
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            usage: wgpu::BufferUsages::INDEX,
            contents: cast_slice(TEST_INDICES),
        });

        let pipelines = vec![
            pipeline::create_pipeline(&device, &shader, &config.format, &[ColoredVertex3D::layout()], Some(&pipeline_layout)),
        ];

        Ok(Self {
            window,
            size,
            surface,
            device,
            queue,
            config,
            pipelines,
            vertex_buffer,
            index_buffer,
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
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..TEST_INDICES.len() as u32, 0, 0..1);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output_texture.present();

        Ok(())
    }

    pub fn window(&self) -> &Window {
        &self.window
    }
}
