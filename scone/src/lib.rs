pub mod engine;
pub mod scene;
pub mod state;

pub use scone_ecs;

pub enum Error {
    EcsError(scone_ecs::EcsError),
}