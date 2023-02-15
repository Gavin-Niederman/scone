mod components;
mod systems;

use scone::scene::Scene;
use scone::state::StateBuilder;
use scone_ecs::{entity::EntityBuilder, world::WorldBuilder};

fn main() {
    let entity = EntityBuilder::new()
        .with_component(components::ExampleComponent { val: 1 })
        .with_component(scone::components::Renderable { test: 0.0 })
        .build();

    let world = WorldBuilder::new()
        .with_entity(entity)
        .with_system(Box::new(systems::ExampleSystem {}))
        .build();

    let state = StateBuilder::new().with_scene(Scene { world }).build();

    scone::start(state);
}
