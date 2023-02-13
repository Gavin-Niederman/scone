use std::time::Instant;

use crate::{renderable::Renderable, scene::Scene};
use saunter::math::{lerp, lerp_instant};

pub struct State {
    scenes: Vec<Scene>,
    current_scene: usize,
}
impl State {
    pub(crate) fn get_scene(&self) -> Result<&Scene, crate::Error> {
        if let Some(scene) = self.scenes.get(self.current_scene) {
            Ok(scene)
        } else {
            Err(crate::Error::InvalidScene)
        }
    }
}
impl saunter::listener::Listener for State {
    type Tick = Tick;
    type Event = winit::event::Event<'static, ()>;

    fn tick(
        &mut self,
        dt: f32,
        events: Vec<saunter::event::Event<Self::Event>>,
        time: std::time::Instant,
    ) -> Result<Self::Tick, saunter::error::SaunterError> {
        if let Some(scene) = self.scenes.get_mut(self.current_scene) {
            scene.world.tick(dt, events).unwrap_or_else(|errors| {
                for error in errors {
                    log::error!("{error}")
                }
            });
        } else {
            log::error!("{}", crate::Error::InvalidScene);
        }

        if let Ok(scene) = self.get_scene() {
            Ok(Tick {
                time,
                renderables: scene.get_rendereables(),
            })
        } else {
            Err(saunter::error::SaunterError::TickError(
                saunter::tick::TickError::CouldNotCreateTick,
            ))
        }
    }
}

pub struct StateBuilder {
    scenes: Vec<Scene>,
    current_scene: Option<usize>,
}
impl StateBuilder {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
            current_scene: None,
        }
    }

    pub fn with_scene(mut self, scene: Scene) -> Self {
        self.scenes.push(scene);
        self
    }

    pub fn with_default_scene(mut self, current_scene: usize) -> Self {
        self.current_scene = Some(current_scene);
        self
    }

    pub fn build(self) -> State {
        State {
            scenes: self.scenes,
            current_scene: self.current_scene.unwrap_or(0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tick {
    pub time: Instant,
    pub renderables: Vec<Renderable>,
}
impl Tick {
    pub fn first(scene: &Scene) -> Self {
        Self {
            time: Instant::now(),
            renderables: scene.get_rendereables(),
        }
    }
}
impl saunter::tick::Tick for Tick {
    fn lerp(a: &Self, b: &Self, t: f32) -> Result<Self, saunter::math::MathError> {
        let renderables = a
            .renderables
            .iter()
            .zip(b.renderables.iter())
            .map(|renderables| Renderable {
                test: lerp(renderables.0.test, renderables.1.test, t),
            })
            .collect();

        Ok(Self {
            time: lerp_instant(a.get_time(), b.get_time(), t)?,
            renderables,
        })
    }

    fn get_time(&self) -> &std::time::Instant {
        &self.time
    }
}
