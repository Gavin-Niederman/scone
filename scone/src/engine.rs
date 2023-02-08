use std::{
    sync::{mpsc::Sender, Arc, RwLock},
    thread,
};

use saunter::{listener::Listener, tick::Ticks};

use crate::state::State;
pub struct Engine {
    tick_loop: saunter::tickloop::Loop<<State as Listener>::Tick, <State as Listener>::Event>,
    event_sender: Sender<saunter::event::Event<winit::event::Event<'static, ()>>>,
    ticks: &'static Arc<RwLock<Ticks<<State as Listener>::Tick>>>,
}
impl Engine {
    pub fn new(state: State) -> Self {
        simplelog::SimpleLogger::init(simplelog::LevelFilter::Debug, simplelog::Config::default())
            .unwrap_or_else(|_| println!("Unable to initialize logger!"));

        let (tick_loop, event_sender, ticks) = saunter::tickloop::Loop::init(
            Box::new(state),
            <State as Listener>::Tick::first(),
            66.0,
        );
        Self {
            tick_loop,
            event_sender,
            ticks,
        }
    }

    pub fn run(&'static mut self) {
        let loop_ticks = self.ticks.clone();
        thread::spawn(|| self.tick_loop.start(loop_ticks));
    }
}
