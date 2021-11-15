use crate::config::{Config, Color};

pub fn run(mut _config: &mut Config, _audio: audioviz::AudioStream, _color_modes: Vec<Color>) -> ! {
    println!("The Windows terminal is currently not supported, as it does not support ANSI escape codes");
    println!("please use the wgpu backend with the '--backend wgpu' parameter");
    std::process::exit(0);
}