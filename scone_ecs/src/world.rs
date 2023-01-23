use std::any::Any;

use crate::{entity::Entity, resource::{Resouce, ResourceType}};

pub struct World {
    entities: Vec<Entity>,
    resources: Vec<Resouce<dyn ResourceType>>
}