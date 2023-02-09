use std::thread;

use saunter::listener::Listener;
use winit::event::{Event, WindowEvent};

use crate::state::State;

pub fn start(state: State) {
    let (tick_loop, event_sender, ticks) = Box::leak(Box::new(saunter::tickloop::Loop::init(
        Box::new(state),
        <State as Listener>::Tick::first(),
        66.0,
    )));

    let loop_ticks = ticks.clone();
    thread::spawn(|| tick_loop.start(loop_ticks));

    let event_loop = winit::event_loop::EventLoop::new();
    let _window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                log::info!("Shutting down!");
                control_flow.set_exit();
            },
            _ => ()
        }

        event_sender
        .send(saunter::event::Event::Other(event.to_static().unwrap_or(
            winit::event::Event::NewEvents(winit::event::StartCause::Poll),
        )))
        .unwrap_or_else(|err| log::error!("{:?}", err));
    })
}