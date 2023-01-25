use scone::state::StateBuilder;
use scone::scene::Scene;

fn main() {
    let state_builder = 
        StateBuilder::new()
        .with_scene(Scene {
            world: scone_ecs::world::World::new(),
        });

    let _state = state_builder.build();
}