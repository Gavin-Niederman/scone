use scone::components::RenderableComponent;
use scone_ecs::system::System;

use crate::components::ExampleComponent;

pub struct ExampleSystem {}
impl System<<scone::state::State as saunter::listener::Listener>::Event> for ExampleSystem {
    fn tick(
        &self,
        entities: &mut Vec<scone_ecs::entity::Entity>,
        _resources: &mut Vec<scone_ecs::resource::Resouce<dyn Send + Sync>>,
        _events: &Vec<
            saunter::event::Event<<scone::state::State as saunter::listener::Listener>::Event>,
        >,
        _dt: f32,
    ) -> Result<(), scone_ecs::EcsError> {
        for entity in entities {
            let mut val = 0.0;
            
            if let Ok(component) = entity.get_component_mut::<ExampleComponent>() {
                log::info!("value is {}", component.val);
                component.val = 1 - component.val;
                val = component.val as f32;
            }

            if let Ok(renderable) = entity.get_component_mut::<RenderableComponent>() {
                renderable.renderable.test = val;
            }
        }

        Ok(())
    }
}
