#![allow(unused)]

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

mod state;
use state::*;

mod timer;
use timer::*;

mod print;
use print::*;

async fn async_run() {
    let event_loop = EventLoop::new().unwrap();
    let mut state = State::new(&event_loop).await;

    let mut time = Timer::new();

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => {
                if !state.input(event) {
                    state.handle_events(event, control_flow);
                    if is_key_pressed(event, KeyCode::Escape) {
                        control_flow.exit();
                    }
                    pr!(time.str_reset());
                }
            }
            _ => {}
        })
        .unwrap();
}

pub fn run() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    pollster::block_on(async_run());
    println!("Done.");
}
