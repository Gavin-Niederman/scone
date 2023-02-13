pub mod components;
pub mod engine;
pub mod scene;
pub mod state;

pub use scone_ecs;

pub use engine::start;

#[derive(Debug)]
pub enum Error {
    EcsError(scone_ecs::EcsError),
    InvalidScene,
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EcsError(err) => writeln!(f, "{err}"),
            Self::InvalidScene => writeln!(f, "Invalid scene selected!"),
        }
    }
}
