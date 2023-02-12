use scone_ecs::component::Component;

use crate::renderable::Renderable;

#[derive(Clone, Copy)]
pub struct RenderableComponent {
    pub renderable: Renderable,
}
impl Component for RenderableComponent {}
