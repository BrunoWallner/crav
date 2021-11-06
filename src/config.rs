use std::f32::consts::PI;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Config {
    pub audio: audioviz::Config,
    pub fps: u64,
    pub color: Color,
    pub width: Width,
    pub mirror: bool,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            audio: audioviz::Config::default(),
            fps: 60,
            color: Color::Rainbow,
            width: Width::Half,
            mirror: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Rainbow,
    Rgb([u8; 3]),
}
impl Color {
    pub fn rainbow_from_y(ry: f32) -> [u8; 3] {
        let ry = ry * PI;
        [
            ((ry).sin() * 230.0) as u8 + 25,
            ((ry + (4.0 * PI / 3.0)).sin() * 230.0) as u8 + 25,
            ((ry + (2.0 * PI / 3.0)).sin() * 230.0) as u8 + 25,
        ]
    } 
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone, Copy)]
pub enum Width {
    Full,
    Half,
}
