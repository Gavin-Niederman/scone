use crate::entity::Entity;

pub trait System {
    type Event;
    
    fn tick(&mut self, entities: &mut Vec<Entity>, event: &mut Self::Event);
}