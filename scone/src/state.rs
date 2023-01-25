use crate::scene::Scene;

pub struct State<'a> {
    scenes: &'a [Scene],
}

pub struct StateBuilder {
    scenes: Vec<Scene>,
}
impl StateBuilder {
    pub fn new() -> Self {
        Self {
            scenes: Vec::new(),
        }
    }

    pub fn with_scene(mut self, scene: Scene) -> Self {
        self.scenes.push(scene);
        self
    }

    pub fn build<'a>(self) -> State<'a> {
        State {
            scenes: Box::leak(self.scenes.into_boxed_slice()),
        }
    }
}