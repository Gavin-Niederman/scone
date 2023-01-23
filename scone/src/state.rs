pub struct State {
    pub ecs: scone_ecs::world::World,
}
impl saunter::listener::Listener for State {
    type TickType = Tick;

    type EventType = winit::event::Event<'static, ()>;

    fn tick(
        &mut self,
        dt: f32,
        events: Vec<saunter::event::Event<Self::EventType>>,
        time: std::time::Instant,
    ) -> Result<Self::TickType, saunter::error::SaunterError> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tick {}
impl saunter::tick::Tick for Tick {
    fn lerp(&self, b: &Self, t: f32) -> Result<Self, saunter::math::MathError> {
        todo!()
    }

    fn get_time(&self) -> &std::time::Instant {
        todo!()
    }
}