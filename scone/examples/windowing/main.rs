mod systems;
mod components;

use scone::scene::Scene;
use scone::state::StateBuilder;
use scone_ecs::{world::WorldBuilder, entity::Entity};

fn main() {
    let mut entity = Entity::new();
    entity.add_component(components::ExampleComponent {val: 1});
    let world = WorldBuilder::new()
        .with_entity(entity)
        .with_system(Box::new(systems::ExampleSystem {}))
        .build();

    let state_builder = StateBuilder::new().with_scene(Scene {
        world,
    });

    let state = state_builder.build();

    scone::start(state)
}
