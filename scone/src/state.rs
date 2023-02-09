use crate::scene::{Scene, Tick};

pub struct State {
    scenes: Vec<Scene>,
    current_scene: usize,
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
        log::info!("ticked");
        Ok(Tick {})
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
