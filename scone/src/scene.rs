use crate::state::State;
use saunter::listener::Listener;

pub struct Scene {
    pub world: scone_ecs::world::World<<State as Listener>::Event>,
}

#[derive(Debug, Clone, Copy)]
pub struct Tick {}
impl Tick {
    pub fn first() -> Self {
        Self {}
    }
}
impl saunter::tick::Tick for Tick {
    fn lerp(&self, b: &Self, t: f32) -> Result<Self, saunter::math::MathError> {
        todo!()
    }

    fn get_time(&self) -> &std::time::Instant {
        todo!()
    }
}
