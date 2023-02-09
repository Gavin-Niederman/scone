use saunter::event::Event;

use crate::{entity::Entity, resource::Resouce, system::System};
use std::marker::PhantomData;

pub struct World<E: Send + Clone> {
    pub entities: Vec<Entity>,
    pub resources: Vec<Resouce<dyn Send + Sync>>,
    pub systems: Vec<Box<dyn System<E>>>,
    phantom: PhantomData<E>,
}

impl<E: Send + Clone> World<E> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            resources: Vec::new(),
            systems: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn tick(&mut self, dt: f32, events: Vec<Event<E>>) -> Result<(), crate::EcsError> {
        for system in self.systems.iter() {
            system.tick(&mut self.entities, &mut self.resources, &events, dt)?;
        }

        Ok(())
    }
}
