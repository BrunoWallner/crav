use std::time::Duration;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct Config {
    pub tick_rate: Duration,
    pub volume: f32,
    pub max_frequency: u16,
    pub fft_resolution: u16,
    pub smoothing_size: u16,
    pub smoothing_amount: u16,
    pub buffering: u16,
    pub color: Color,
    pub width: Width,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            tick_rate: Duration::from_millis(16),
            volume: 5.0,
            max_frequency: 10_000,
            fft_resolution: 6000,
            smoothing_size: 2,
            smoothing_amount: 2,
            buffering: 7,
            color: Color::Rainbow,
            width: Width::Half,
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
pub enum Width {
    Full,
    Half,
}