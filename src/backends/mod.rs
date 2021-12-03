use crate::config::Config;

// IDK how to only use 1 #[cfg] per target_family but this should work at least fine
#[cfg(target_family = "unix")]
mod termion;
#[cfg(target_family = "unix")]
use self::termion as terminal;

#[cfg(target_family = "windows")]
mod crossterm;
#[cfg(target_family = "windows")]
use self::crossterm as terminal;

mod wgpu;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum GridPixel {
    Bar(u8),
    Char(char),
}

pub enum Backend {
    Terminal,
    Wgpu,
}
impl Backend {
    pub fn run(&self, config: &mut Config, audio_controller: audioviz::AudioStreamController) {
        match self {
            Backend::Terminal => {
                terminal::run(config, audio_controller);
            }
            Backend::Wgpu => {
                wgpu::run(config, audio_controller);
            }
        }
    }
}

pub fn gen_grid(x_size: u16, y_size: u16, data: &Vec<audioviz::Frequency>) -> Vec<Vec<GridPixel>> {
    let mut buffer: Vec<Vec<GridPixel>> = vec![vec![GridPixel::Bar(0); x_size as usize]; y_size as usize];

    // bars
    for y in 0..y_size as usize {
        for x in 0..x_size as usize {
            for r in 0..8 {
                if data.len() > x {
                    let exact_y: f32 = ((y + 1) as f32 / y_size as f32) + (r as f32 * 0.125) / y_size as f32;
                    if data[x].volume >= exact_y {
                        buffer[y][x] = GridPixel::Bar(r + 1);
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
