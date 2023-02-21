use crate::{shader, vertex::Vertex};

pub trait Rendereable {
    type Vertex: Vertex;

    fn get_vertices(&self) -> &[Self::Vertex];

    fn get_indices(&self) -> &[u32];

    fn get_shadertype() -> shader::ShaderType {
        Self::Vertex::shader_type()
    }
}
