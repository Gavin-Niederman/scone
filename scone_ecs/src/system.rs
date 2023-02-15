use crate::{entity::Entity, resource::Resouce};

pub trait System<E: Send + Clone + ?Sized>: Send {
    fn tick(
        &self,
        entities: &mut Vec<Entity>,
        resources: &mut Vec<Resouce<dyn Send + Sync>>,
        events: &[saunter::event::Event<E>],
        dt: f32,
    ) -> Result<(), crate::EcsError>;
}

// Eventually I would like to implement this on closures but I can't right now.
//
// impl<E: Send + Clone + ?Sized> System<E>
//     for dyn Fn(
//         &mut Vec<Entity>,
//         &mut Vec<Resouce<dyn Send + Sync>>,
//         &Vec<saunter::event::Event<E>>,
//         f32,
//     ) -> Result<(), crate::EcsError>
// {
//     fn tick(
//         &self,
//         entities: &mut Vec<Entity>,
//         resources: &mut Vec<Resouce<dyn Send + Sync>>,
//         events: &Vec<saunter::event::Event<E>>,
//         dt: f32,
//     ) -> Result<(), crate::EcsError> {
//         self(entities, resources, events, dt)
//     }
// }
