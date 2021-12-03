mod state;
mod mesh;

use state::Vertex;
use crate::backends::get_bar_number;

use crate::config::{Config};

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Fullscreen,
};
use winit_input_helper::WinitInputHelper;


pub const PIXEL_WIDTH: u16 = 2;
pub const PIXEL_HEIGHT: u16 = 18;

pub fn run(config: &mut Config, audio_controller: audioviz::AudioStreamController) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_transparent(config.wgpu.transparent)
        .with_decorations(config.wgpu.decoration)
        .with_title("crav")
        .build(&event_loop)
        .unwrap();

    if config.wgpu.fullscreen {
        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    }
    let a_c = audio_controller.clone();

    let mut state = pollster::block_on(state::State::new(&window, a_c, config.clone() ));

    let mut input = WinitInputHelper::new();
    let config = config.clone();
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Plus) || input.key_pressed(VirtualKeyCode::NumpadAdd) {
                audio_controller.adjust_volume(1.1);
            }
            if input.key_pressed(VirtualKeyCode::Minus) || input.key_pressed(VirtualKeyCode::NumpadSubtract) {
                audio_controller.adjust_volume(0.9);
            }
            if input.key_pressed(VirtualKeyCode::M) {
                state.config.mirror = !state.config.mirror;

                let screen_width = state.size.width as u16 / PIXEL_WIDTH;
                let mut bar_number = get_bar_number(config.width, config.spacing, screen_width) as usize;
                if state.config.mirror {bar_number /= 2}
                audio_controller.set_resolution(bar_number);
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
