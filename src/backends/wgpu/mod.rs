mod state;
mod mesh;

use state::Vertex;

use std::error::Error;
use crate::config::{Config, Color, Width};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use std::sync::mpsc;


pub const PIXEL_WIDTH: u16 = 18;
pub const PIXEL_HEIGHT: u16 = 18;

pub fn run(config: &mut Config, audio: audioviz::AudioStream, color_modes: Vec<Color>) -> Result<(), Box<dyn Error>> {
    let audio_ev = audio.get_event_sender();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut state = pollster::block_on(state::State::new(&window, audio, config.clone() ));

    let cm = color_modes.into_iter();
    let mut color_modes = cm.cycle();

    let mut input = WinitInputHelper::new();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::C) {
                state.config.color = color_modes.next().unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Plus) || input.key_pressed(VirtualKeyCode::NumpadAdd) {
                let (tx, rx) = mpsc::channel();
                audio_ev.send(audioviz::Event::RequestConfig(tx)).unwrap();
                let cfg = rx.recv().unwrap();
                let config = audioviz::Config {
                    volume: cfg.volume * 1.1,
                    ..cfg
                };
                audio_ev.send(audioviz::Event::SendConfig(config)).unwrap();
            }
            if input.key_pressed(VirtualKeyCode::Minus) || input.key_pressed(VirtualKeyCode::NumpadSubtract) {
                let (tx, rx) = mpsc::channel();
                audio_ev.send(audioviz::Event::RequestConfig(tx)).unwrap();
                let cfg = rx.recv().unwrap();
                let config = audioviz::Config {
                    volume: cfg.volume * 0.9,
                    ..cfg
                };
                audio_ev.send(audioviz::Event::SendConfig(config)).unwrap();
            }
            if input.key_pressed(VirtualKeyCode::W) {
                state.config.width = match state.config.width {
                    Width::Full => Width::Half,
                    Width::Half => Width::Full, 
                }
            }
        }


        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !state.input(event) { // UPDATED!
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Q),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            winit::event::Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            winit::event::Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
