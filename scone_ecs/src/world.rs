use std::any::Any;

use crate::{entity::Entity, resource::Resouce};

pub struct World {
    entities: Vec<Entity>,
    resources: Vec<Resouce<dyn Any>>
}