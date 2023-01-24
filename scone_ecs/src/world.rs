use crate::{entity::Entity, resource::Resouce};

pub struct World {
    entities: Vec<Entity>,
    resources: Vec<Resouce<dyn Send>>,
}

impl World {
    // pub fn tick(&mut self, dt: f32, event: T) {
    //     todo!()
    // }
}
