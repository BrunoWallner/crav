use crate::config::Config;

//use audioviz::spectrum::stream::StreamController;
use audioviz::spectrum::Frequency;

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
pub mod audio_to_grid;

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
    pub fn run(&self, config: &mut Config, converter: audio_to_grid::Converter) {
        match self {
            Backend::Terminal => {
                terminal::run(config, converter);
            }
            Backend::Wgpu => {
                wgpu::run(config, converter);
            }
        }
    }
}

pub fn get_bar_number(width: u8, spacing: u8, screen_width: u16) -> usize {
    if width == 0 {return 1}
    (screen_width / (width + spacing) as u16) as usize
}
