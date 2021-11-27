use crate::backends::wgpu::Vertex;
use crate::config::{Config, Color};
use crate::backends::{gen_grid, GridPixel};

use crate::backends::wgpu::{PIXEL_WIDTH, PIXEL_HEIGHT};

pub fn from_buffer(
    buffer: Vec<audioviz::Frequency>,
    config: &Config,
    window_size: (u32, u32),
) -> (Vec<Vertex>, Vec<u32>)  {
    let (w, h) = ( (window_size.0 / PIXEL_WIDTH as u32) as u16, (window_size.1 / PIXEL_HEIGHT as u32) as u16 );
    let width = 1.0 / w as f32 * config.width as f32 * 2.0; // * 2.0 because wgpu goes from -1 to 1

    //let w: u16 = get_bar_number(config.width, config.spacing, w) as u16; // calculates width further

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

    for y in 0..h as usize {
        let color_clone = config.color.clone();
        let color: [f32; 3] = match color_clone {
            Color::Rgb(c) => [ 
                    (c[0] as f32 / 255.0).powf(2.2),
                    (c[1] as f32 / 255.0).powf(2.2),
                    (c[2] as f32 / 255.0).powf(2.2),
                ],
            c => {
                let c = c.to_rgb(y as f32 / h as f32);
                [ 
                    (c[0] as f32 / 255.0).powf(2.2),
                    (c[1] as f32 / 255.0).powf(2.2),
                    (c[2] as f32 / 255.0).powf(2.2),
                ]
            }
        };

        for x in 0..w as usize {
            let p: f32 = match grid[y][x] {
                GridPixel::Bar(bar_height) => {
                    bar_height as f32 * (1.0 / h as f32) / 8.0 * 2.0
                }
                GridPixel::Freq(_) => 0.0
            };

            let x = ((x as f32 / w as f32) * (config.spacing + config.width) as f32)
                * 2.0 - 1.0; // because wgpu goes from -1 to 1
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
     
    return (vertices, indices);
}
