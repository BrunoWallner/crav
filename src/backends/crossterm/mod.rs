use crate::config::{Config, Color};
use audioviz::spectralizer::stream::StreamController;

pub fn run(mut _config: &mut Config, _audio_controller: StreamController) {
    println!("The Windows terminal is currently not supported, as it does not support ANSI escape codes");
    println!("please use the wgpu backend with the '--backend wgpu' parameter");
}