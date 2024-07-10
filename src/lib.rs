#![allow(unused)]

use winit::{event::*, event_loop::EventLoop, keyboard::KeyCode};

mod state;

async fn async_run() {
    let event_loop = EventLoop::new().unwrap();
    let mut state = state::State::new(&event_loop).await;

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => {
                state.handle_events(event, control_flow);
            }
            _ => {}
        })
        .unwrap();
}

pub fn run() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    pollster::block_on(async_run());
}
