use std::time::Instant;

use crate::state::State;
use saunter::listener::Listener;
use saunter::math::{lerp, lerp_instant};

pub struct Scene {
    pub world: scone_ecs::world::World<<State as Listener>::Event>,
}

#[derive(Debug, Clone, Copy)]
pub struct Tick {
    pub time: Instant,
}
impl Tick {
    pub fn first() -> Self {
        Self {
            time: Instant::now(),
        }
    }
}
impl saunter::tick::Tick for Tick {
    fn lerp(a: &Self, b: &Self, t: f32) -> Result<Self, saunter::math::MathError> {
        Ok(
            Self {
                time: lerp_instant(a.get_time(), b.get_time(), t)?,
            }
        )
    }

    fn get_time(&self) -> &std::time::Instant {
        &self.time
    }
}
