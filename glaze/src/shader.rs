use std::path::PathBuf;

pub enum ShaderType {
    VertexColor,
    Textured,
    Custom(Shader),
}

pub struct Shader {
    pub path: PathBuf,
    pub(crate) shader_module: wgpu::ShaderModule
}
