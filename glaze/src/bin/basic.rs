use glaze::State;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let mut state = State::new(window);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        state.handle_event(&event);

        match event {
            winit::event::Event::WindowEvent { ref event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                _ => {}
            },
            _ => {}
        }
    });
}
