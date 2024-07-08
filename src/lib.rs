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

async fn async_run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(&window).await;

    let mut time = Timer::new();

    event_loop
        .run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => control_flow.exit(),

                        WindowEvent::Resized(physical_size) => {
                            log::info!("Resized: {physical_size:?}");
                            state.surface_configured = true;
                            // TODO: what sets surface_configured to false?
                            state.resize(*physical_size);
                        }

                        WindowEvent::RedrawRequested => state.request_redraw(control_flow),

                        _ => {
                            if is_key_pressed(event, KeyCode::Escape) {
                                control_flow.exit();
                            }

                            println!("{}", time.str_reset());

                            // std::thread::sleep(std::time::Duration::from_millis(16));
                            // OF COURSE!! this function is called multiple times. so most calls are very quick
                        }
                    }
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
