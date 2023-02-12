use scone_ecs::system::System;

use crate::components::ExampleComponent;

pub struct ExampleSystem {}
impl System<<scone::state::State as saunter::listener::Listener>::Event> for ExampleSystem {
    fn tick(
        &self,
        entities: &mut Vec<scone_ecs::entity::Entity>,
        resources: &mut Vec<scone_ecs::resource::Resouce<dyn Send + Sync>>,
        events: &Vec<saunter::event::Event<<scone::state::State as saunter::listener::Listener>::Event>>,
        dt: f32,
    ) -> Result<(), scone_ecs::EcsError> {
        for entity in entities {
            if let Ok(component) = entity.get_component::<ExampleComponent>() {
                log::info!("value is {}", component.val);
                component.val += 1;
            }
        }

        Ok(())
    }
}