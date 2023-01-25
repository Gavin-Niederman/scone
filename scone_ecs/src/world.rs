use crate::{entity::Entity, resource::Resouce};
use std::marker::PhantomData;

pub struct World<T: Send> {
    entities: Vec<Entity>,
    resources: Vec<Resouce<dyn Send>>,
    phantom_data: PhantomData<T>,
}

impl<T: Send> World<T> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            resources: Vec::new(),
            phantom_data: PhantomData,
        }
    }

    pub fn tick(&mut self, dt: f32, events: Vec<T>) {
        todo!()
    }
}
