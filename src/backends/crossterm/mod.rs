pub fn run(mut _config: &mut Config, _audio: audioviz::AudioStream, _color_modes: Vec<Color>) {
    println!("The Windows terminal is currently not supported, as it does not support ansi terminals, please use the wgpu backend with the '--backend wgpu' parameter")
}