use crate::backends::wgpu::Vertex;
use crate::config::{Config, Color, Width};
use crate::backends::gen_grid;

use crate::backends::wgpu::{PIXEL_WIDTH, PIXEL_HEIGHT};

pub fn from_buffer(
    buffer: Vec<f32>,
    config: &Config,
    window_size: (u32, u32),
) -> (Vec<Vertex>, Vec<u32>)  {
    let (w, h) = ( (window_size.0 / PIXEL_WIDTH as u32) as u16, (window_size.1 / PIXEL_HEIGHT as u32) as u16 );

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    if buffer.len() == 0 {
        return (Vec::new(), Vec::new());
    }

    let grid = gen_grid(
            w,
            h,
        &buffer, 
    );

    let width = match config.width {
        Width::Half => PIXEL_WIDTH as f32 / window_size.0 as f32 * 1.0,
        Width::Full => PIXEL_WIDTH as f32 / window_size.0 as f32 * 2.1, // to compensate for some odd spacing
    };

    for y in 0..h as usize {
        let color: [f32; 3] = match config.color {
            Color::Rgb(c) => [ 
                    (c[0] as f32 / 255.0).powf(2.2),
                    (c[1] as f32 / 255.0).powf(2.2),
                    (c[2] as f32 / 255.0).powf(2.2),
                ],
            Color::Rainbow => {
                let c = Color::rainbow_from_y(y as f32 / h as f32);
                [ 
                    (c[0] as f32 / 255.0).powf(2.2),
                    (c[1] as f32 / 255.0).powf(2.2),
                    (c[2] as f32 / 255.0).powf(2.2),
                ]
            }
        };

        for x in 0..w as usize {
            if grid[y][x] > 0 && grid[y][x] <= 8 {
                let p = grid[y][x] as f32 * (1.0 / h as f32) / 8.0 * 2.0;

                let x = x as f32 / w as f32 * 2.0 - 1.0;
                let y = y as f32 / h as f32 * 2.0 - 1.0;
    
                vertices.push(Vertex { position: [x,  y, 0.0],   color});
                vertices.push(Vertex { position: [x + width,  y, 0.0],   color});
    
                vertices.push(Vertex { position: [x,  y + p, 0.0],   color});
                vertices.push(Vertex { position: [x + width,  y + p, 0.0],   color});
    
                let i = vertices.len() as u32 - 4;
                indices.push(i+0);
                indices.push(i+3);
                indices.push(i+2);
                indices.push(i+0);
                indices.push(i+1);
                indices.push(i+3);
            }
        }
    }
     
    return (vertices, indices);
}
