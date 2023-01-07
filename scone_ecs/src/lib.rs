pub mod entity;
pub mod component;
pub mod system;

#[derive(Debug)]
pub enum EcsError {
    ComponentNotFound(&'static str),
}
impl std::error::Error for EcsError {}
impl std::fmt::Display for EcsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComponentNotFound(name) => write!(f, "Component {} not found", name),
        }
    }
}
