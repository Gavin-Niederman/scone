use scone::scene::Scene;
use scone::state::StateBuilder;

fn main() {
    let state_builder = StateBuilder::new().with_scene(Scene {
        world: scone_ecs::world::World::new(),
    });

    let state = state_builder.build();

    let scone = Box::leak(Box::new(scone::engine::Engine::new(state)));

    scone.run()
}
