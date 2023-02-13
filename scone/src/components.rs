use scone_ecs::component::Component;

#[derive(Clone, Copy, Debug)]
pub struct Renderable {
    pub test: f32,
}
impl Component for Renderable {}
