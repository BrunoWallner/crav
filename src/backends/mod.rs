use crate::config::Config;

use audioviz::spectrum::stream::StreamController;
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
    pub fn run(&self, config: &mut Config, audio_controller: StreamController) {
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

pub fn gen_grid(x_size: u16, y_size: u16, data: &Vec<Frequency>, width: u8, spacing: u8) -> Vec<Vec<GridPixel>> {
    let mut buffer: Vec<Vec<GridPixel>> = vec![vec![GridPixel::Bar(0); x_size as usize]; y_size as usize];

    /* bars
    for y in 0..y_size as usize {
        for x in 0..x_size as usize {
            for r in 0..8 {
                if data.len() > x {
                    let exact_y: f32 = (y as f32 / y_size as f32) + (r as f32 * 0.125) / y_size as f32;
                    if data[x].volume >= exact_y {
                        buffer[y][x] = GridPixel::Bar(r + 1);
                    }
                }
            }
        }
    }
    */

    /*
    let mut screen_x: usize = 0;
    let mut x: usize = 0;

    let mut y: usize = 0;
    'y: loop {
        'x: loop {
            let rel_y: f32 = y as f32 / y_size as f32;
            for _ in 0..width {
                screen_x += 1;
                if screen_x as u16 >= x_size || x >= data.len() {break 'x}
                if data[x].volume >= rel_y {
                    buffer[y][screen_x] = GridPixel::Bar(8);
                }
            }
            x += 1;
            screen_x += spacing as usize;
        }
        x = 0;
        screen_x = 0;
        y += 1;
        if y as u16 >= y_size {break 'y}
    }
    */
    let mut screen_x: usize = 0;
    //let mut x: usize = 0;
    for x in 0..x_size as usize {
        if data.len() > x {
            let height: usize = data[x].volume.trunc() as usize;

            // can range from 0 to 1, top of bar for 8 times more precision
            let precision_top: f32 = data[x].volume - height as f32;
            let upper_bar: u8 = (precision_top * 8.0) as u8 + 1;

            for _ in 0..width {
                for y in 0..height {
                    if buffer.len() > y && buffer[y].len() > screen_x {
                        buffer[y][screen_x] = GridPixel::Bar(8);
                    }
                }

                // precision top bar
                if buffer.len() > height && buffer[height].len() > screen_x {
                    buffer[height][screen_x] = GridPixel::Bar(upper_bar);
                }
                screen_x += 1;
            }
            screen_x += spacing as usize;
        }
    }

    buffer
}

pub fn get_bar_number(width: u8, spacing: u8, screen_width: u16) -> usize {
    if width == 0 {return 1}
    (screen_width / (width + spacing) as u16) as usize
}
