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


#[derive(Clone, Copy, Debug)]
pub enum GridPixel {
    Bar(u8),
    Freq(u8),
}

pub enum Backend {
    Terminal,
    Wgpu,
}
impl Backend {
    pub fn run(&self, config: &mut Config, audio: audioviz::AudioStream) {
        match self {
            Backend::Terminal => {
                terminal::run(config, audio);
            }
            Backend::Wgpu => {
                wgpu::run(config, audio);
            }
        }
    }
}

pub fn gen_grid(x_size: u16, y_size: u16, data: &Vec<audioviz::Frequency>) -> Vec<Vec<GridPixel>> {
    let mut buffer: Vec<Vec<GridPixel>> = vec![vec![GridPixel::Bar(0); x_size as usize]; y_size as usize];
    let f_d_p: usize = 5; //freq_display_precicion

    // bars
    for y in 0..y_size as usize - f_d_p { // for 5 decimal place freq displaying
        for x in 0..x_size as usize {
            for r in 0..8 {
                if data.len() > x {
                    let exact_y: f32 = ((y + 1) as f32 / y_size as f32) + (r as f32 * 0.125) / y_size as f32;
                    if data[x].volume >= exact_y {
                        buffer[y + f_d_p][x] = GridPixel::Bar(r + 1);
                    }
                }
            }
        }
    }

    // freqs
    for x in 0..x_size as usize {
        if data.len() > x {
            let freq: Vec<u8> = 
            data[x].freq
                .floor()
                .to_string()
                .chars()
                .map(|c| match c.to_string().parse::<u8>() {
                    Ok(u) => u,
                    Err(_) => 10, // only ok because of checking if it is less than 10 later
                })
                .collect();
    
            for f in 0..f_d_p { // for 5 decimal place freq displaying
                if freq.len() > f && freq[f] < 10 {
                    buffer[f_d_p - (f + 1)][x] = GridPixel::Freq(freq[f])
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
