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
    pub fn tick(&mut self, dt: f32, events: Vec<Event<E>>) -> Result<(), Vec<crate::EcsError>> {
        let errors: Vec<crate::EcsError> = self.systems.iter().filter_map(|system| system.tick(&mut self.entities, &mut self.resources, &events, dt).err()).collect();
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }
}

pub struct WorldBuilder<E: Send + Clone> {
    pub entities: Vec<Entity>,
    pub resources: Vec<Resouce<dyn Send + Sync>>,
    pub systems: Vec<Box<dyn System<E>>>,
    phantom: PhantomData<E>,
}
impl<E: Send + Clone> WorldBuilder<E> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            resources: Vec::new(),
            systems: Vec::new(),
            phantom: PhantomData,
        }
    }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entities.push(entity);
        self
    }
    pub fn with_entities(mut self, mut entities: Vec<Entity>) -> Self {
        self.entities.append(&mut entities);
        self
    }

    pub fn with_resource(mut self, resource: Resouce<dyn Send + Sync>) -> Self {
        self.resources.push(resource);
        self
    }
    pub fn with_resources(mut self, mut resources: Vec<Resouce<dyn Send + Sync>>) -> Self {
        self.resources.append(&mut resources);
        self
    }

    pub fn with_system(mut self, system: Box<dyn System<E>>) -> Self {
        self.systems.push(system);
        self
    }
    pub fn with_systems(mut self, mut systems: Vec<Box<dyn System<E>>>) -> Self {
        self.systems.append(&mut systems);
        self
    }

    pub fn build(self) -> World<E> {
        World {
            entities: self.entities,
            resources: self.resources,
            systems: self.systems,
            phantom: PhantomData,
        }
    }
}
