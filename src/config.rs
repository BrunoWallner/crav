use serde::{Serialize, Deserialize};
use splines::{Interpolation, Key, Spline}; // for interpolation in color gradients

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Config {
    pub audio: audioviz::Config,
    pub fps: u64,
    pub color: Color,
    pub width: u8,
    pub spacing: u8,
    pub mirror: bool,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            audio: audioviz::Config::default(),
            fps: 60,
            color: Color::Gradient(vec![ [255, 0, 0], [0, 255, 0], [0, 0, 255] ]),
            width: 4,
            spacing: 1,
            mirror: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub enum Color {
    Gradient(Vec<[u8; 3]>),
    Rgb([u8; 3]),
}
impl Color {
    pub fn to_rgb(&self, relative_y: f32) -> [u8; 3] {
        match self {
            // It may be unclean to recreate the spline everytime, but I want to avoid to rewrite too much for
            // every backend, and the performance penalty is not too big. (0µs, 0% cpu usage difference)
            Color::Gradient(g) => {
                let mut r_points: Vec<Key<f32, f32>> = Vec::new();
                let mut g_points: Vec<Key<f32, f32>> = Vec::new();
                let mut b_points: Vec<Key<f32, f32>> = Vec::new();

                let step: f32 = 1.0_f32 / g.len() as f32;
                for (i, color) in g.iter().enumerate() {
                    r_points.push(Key::new( (i + 1) as f32 * step, color[0] as f32, Interpolation::Linear ));
                    g_points.push(Key::new( (i + 1) as f32 * step, color[1] as f32, Interpolation::Linear ));
                    b_points.push(Key::new( (i + 1) as f32 * step, color[2] as f32, Interpolation::Linear ));
                }
                let r_spline = Spline::from_vec(r_points);
                let g_spline = Spline::from_vec(g_points);
                let b_spline = Spline::from_vec(b_points);

                [
                    r_spline.clamped_sample(relative_y).unwrap_or(0.0) as u8,
                    g_spline.clamped_sample(relative_y).unwrap_or(0.0) as u8,
                    b_spline.clamped_sample(relative_y).unwrap_or(0.0) as u8
                ]
            }
            Color::Rgb(c) => *c
        }
    }
}
/* cool sort of crt effect

((ry).sin() * 230.0) as u8 + 25,
((ry + (2.0 * PI)).sin() * 230.0) as u8 + 25,
((ry + (4.0 * PI)).sin() * 230.0) as u8 + 25,

*/
