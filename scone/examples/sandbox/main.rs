mod components;
mod systems;

use scone::scene::Scene;
use scone::state::StateBuilder;
use scone_ecs::{entity::Entity, world::WorldBuilder};

fn main() {
    let mut entity = Entity::new();
    entity.add_component(components::ExampleComponent { val: 1 });
    entity.add_component(scone::components::Renderable { test: 0.0 });

    let world = WorldBuilder::new()
        .with_entity(entity)
        .with_system(Box::new(systems::ExampleSystem {}))
        .build();

    let state_builder = StateBuilder::new().with_scene(Scene { world });

    let state = state_builder.build();

    scone::start(state)
}
