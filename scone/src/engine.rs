use std::thread;

use saunter::listener::Listener;
use winit::event::{Event, WindowEvent};

use crate::state::State;

pub fn start(state: State) {
    simplelog::SimpleLogger::init(simplelog::LevelFilter::Info, simplelog::Config::default())
        .unwrap_or_else(|_| println!("Unable to initialize logger"));

    let (tick_loop, event_sender, ticks) = Box::leak(Box::new(saunter::tickloop::Loop::init(
        Box::new(state),
        <State as Listener>::Tick::first(),
        66.0,
    )));

    let loop_ticks = ticks.clone();
    thread::spawn(|| tick_loop.start(loop_ticks));

    let event_loop = winit::event_loop::EventLoop::new();
    let _window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                log::info!("Shutting down!");
                control_flow.set_exit();
            }
            _ => (),
        }

        if let Some(static_event) = event.to_static() {
            event_sender
                .send(saunter::event::Event::Other(static_event))
                .unwrap_or_else(|err| log::error!("{:?}", err));
        }
    })
}
