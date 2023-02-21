use std::mem::size_of;

use crate::shader;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColoredVertex3D {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
impl Vertex for ColoredVertex3D {
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<ColoredVertex3D>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: size_of::<[f32; 3]>() as u64,
                    shader_location: 1,
                },
            ],
        }
    }

    fn shader_type() -> shader::ShaderType {
        shader::ShaderType::VertexColor
    }
}

pub trait Vertex {
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a>;

    fn shader_type() -> shader::ShaderType;
}
