use std::thread;

use saunter::{listener::Listener, tick::Tick};
use winit::event::{Event, WindowEvent};

use crate::state::State;

const TPS: f32 = 66.0;

pub fn start(state: State) {
    simplelog::SimpleLogger::init(simplelog::LevelFilter::Info, simplelog::Config::default())
        .unwrap_or_else(|_| println!("Unable to initialize logger"));

    let first_tick = <State as Listener>::Tick::first(state.get_scene().unwrap().clone());

    let (tick_loop, event_sender, ticks) = Box::leak(Box::new(saunter::tickloop::Loop::init(
        Box::new(state),
        first_tick,
        TPS,
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
                event_sender.send(saunter::event::Event::Close).unwrap();
                control_flow.set_exit();
            }
            _ => (),
        }

        if let Some(static_event) = event.to_static() {
            event_sender
                .send(saunter::event::Event::Other(static_event))
                .unwrap_or_else(|err| log::error!("{:?}", err));
        }

        let read_ticks = ticks.read().unwrap();

        if let Some(last) = &read_ticks.last_tick {
            let mapped_t = saunter::math::max(
                (last.get_time().elapsed().as_secs_f32() * TPS as f32) - 1.0, //subtract 1 to get the previous tick
                0.0,
            );
            if let Ok(lerped) = read_ticks.lerp(mapped_t) {
                for renderable in lerped.renderables {
                    log::info!("{}", renderable.test)
                }
            }
        }
    })
}
