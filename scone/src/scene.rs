use crate::components::Renderable;
use crate::state::State;
use saunter::listener::Listener;

pub struct Scene {
    pub world: scone_ecs::world::World<<State as Listener>::Event>,
}
impl Scene {
    pub fn get_rendereables(&self) -> Vec<Renderable> {
        self.world
            .entities
            .iter()
            .filter(|entity| entity.has_component::<Renderable>())
            .map(|entity| {
                entity
                    .get_component::<Renderable>()
                    .unwrap()
                    .clone()
            })
            .collect()
    }
}
