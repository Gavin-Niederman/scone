use std::thread;

use saunter::{listener::Listener, tick::Tick};
use winit::event::{Event, WindowEvent};
use clap::Parser;

use crate::state::State;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "This game was written with Scone")]
struct Args {
    /// The number of ticks to run per second
    #[arg(short, long, default_value = "66.0")]
    tps: f32,
}

pub fn start(state: State) {
    let args = Args::parse();

    simplelog::SimpleLogger::init(simplelog::LevelFilter::Info, simplelog::Config::default())
        .unwrap_or_else(|_| println!("Unable to initialize logger"));

    let first_tick = <State as Listener>::Tick::first(state.get_scene().unwrap());

    let (tick_loop, event_sender, ticks) = Box::leak(Box::new(saunter::tickloop::Loop::init(
        Box::new(state),
        first_tick,
        args.tps,
    )));

    let loop_ticks = ticks.clone();
    thread::spawn(|| tick_loop.start(loop_ticks));

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut renderer = glaze::State::new(window).unwrap();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => {
                    log::info!("Shutting down!");
                    event_sender.send(saunter::event::Event::Close).unwrap();
                    control_flow.set_exit();
                },
                WindowEvent::Resized(new_size) => {
                    renderer.resize(*new_size).unwrap_or_else(|err| log::error!("{err}"));
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == renderer.window().id() => {

                if let Some(static_event) = event.to_static() {
                    event_sender
                        .send(saunter::event::Event::Other(static_event))
                        .unwrap_or_else(|err| log::error!("{:?}", err));
                }
        
                let read_ticks = ticks.read().unwrap();
        
                if let Some(last) = &read_ticks.last_tick {
                    let mapped_t = saunter::math::max(
                        (last.get_time().elapsed().as_secs_f32() * args.tps) - 1.0, //subtract 1 to get the previous tick
                        0.0,
                    );
                    if let Ok(lerped) = read_ticks.lerp(mapped_t) {
                        for _renderable in lerped.renderables {
                            // Do something with the renderer here
                        }
                    }
                }

                renderer.update();

                match renderer.render() {
                    Err(wgpu::SurfaceError::Lost) => renderer.reconfigure(),
                    Err(wgpu::SurfaceError::OutOfMemory) => control_flow.set_exit(),
                    Err(err) => log::error!("{err}"),
                    _ => {}

                }
            }
            _ => (),
        }
    })
}
