pub mod component;
pub mod entity;
pub mod resource;
pub mod world;

#[derive(Debug)]
pub enum EcsError {
    ComponentNotFound(&'static str),
}
impl std::error::Error for EcsError {}
impl std::fmt::Display for EcsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComponentNotFound(name) => write!(f, "Component {name} not found"),
        }
    }
}
