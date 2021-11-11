use crate::config::Config;
use crate::config::Color;

mod termion;
mod wgpu;

pub enum Backend {
    Terminal,
    Wgpu,
}
impl Backend {
    pub fn run(&self, config: &mut Config, audio: audioviz::AudioStream, color_modes: Vec<Color>) {
        match self {
            Backend::Terminal => {
                termion::run(config, audio,  color_modes).unwrap();
            }
            Backend::Wgpu => {
                wgpu::run(config, audio, color_modes).unwrap();
            }
        }
    }
}

pub fn gen_grid(x_size: u16, y_size: u16, data: &Vec<f32>) -> Vec<Vec<u8>> {
    let mut buffer: Vec<Vec<u8>> = vec![vec![0; x_size as usize]; y_size as usize];
    for y in 0..y_size as usize {
        for x in 0..x_size as usize {
            for r in 0..8 {
                if data.len() > x {
                    let exact_y: f32 = ((y + 1) as f32 / y_size as f32) + (r as f32 * 0.125) / y_size as f32;
                    if data[x] >= exact_y {
                        buffer[y][x] = r + 1;
                    }
                }
            }
        }
    }
    buffer
}

pub fn get_bar_number(width: u8, spacing: u8, screen_width: u16) -> usize {
    if width == 0 {return 1}
    (screen_width / (width + spacing) as u16) as usize
}
