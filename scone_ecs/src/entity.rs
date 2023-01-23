use std::any::Any;
use crate::{component::Component, EcsError};

pub struct Entity {
    components: Vec<Box<dyn Any + Send>>,
}
impl Entity {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_component<T: Component + 'static + Send>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }

    pub fn get_component<T: Component + 'static>(&mut self) -> Result<&mut T, EcsError> {
        if let Some(component) = self.components.iter_mut().find_map(|c| c.downcast_mut::<T>()) {
            Ok(component)
        } else {
            Err(EcsError::ComponentNotFound(std::any::type_name::<T>()))
        }
    }

    pub fn has_component<T: Component + 'static>(&self) -> bool {
        self.components.iter().any(|c| c.is::<T>())
    }
}
