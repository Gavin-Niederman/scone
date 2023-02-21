pub mod pipeline;
pub mod renderable;
pub mod shader;
pub mod state;
pub mod vertex;

pub use state::State;

#[derive(Debug)]
pub enum Error {
    Init(InitError),
    Misc(MiscError),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Init(err) => writeln!(f, "{err}"),
            Self::Misc(err) => writeln!(f, "{err}"),
        }
    }
}

#[derive(Debug)]
pub enum InitError {
    NoSupportedAdapter,
    SurfaceLacksCapabilities,
    SurfaceIncompatible,
}
impl std::error::Error for InitError {}
impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSupportedAdapter => writeln!(f, "No supported adapters found"),
            Self::SurfaceLacksCapabilities => writeln!(f, "The window lacks required capabilities"),
            Self::SurfaceIncompatible => {
                writeln!(f, "The window is incompatible with your graphics card")
            }
        }
    }
}

#[derive(Debug)]
pub enum MiscError {
    WindowTooSmall,
}
impl std::error::Error for MiscError {}
impl std::fmt::Display for MiscError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowTooSmall => writeln!(f, "Window size too small to render to"),
        }
    }
}
