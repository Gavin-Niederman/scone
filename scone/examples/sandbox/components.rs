use scone_ecs::component::Component;

#[derive(Clone, Copy)]
pub struct ExampleComponent {
    pub val: i32,
}
impl Component for ExampleComponent {}

#[derive(Clone, Copy)]
pub struct ErrorComponent {}
impl Component for ErrorComponent {}
