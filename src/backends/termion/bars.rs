use std::io::{StdoutLock, Write};
use std::error::Error;
use termion::color;
use crate::config::Color;
use std::io::BufWriter;
use crate::backends::GridPixel;

use audioviz::spectrum::Frequency;

fn get_lines(width: u16, height: u16, grid: Vec<Vec<GridPixel>>, color: Color) -> Vec<String> {
    let mut lines: Vec<String> = vec![String::new(); height as usize];
    //let calculated_width: usize = get_bar_number(w, spacing, width) * 2;

    match color {
        Color::Rgb(color) => {
            lines[0].push_str( &format!("{}", color::Fg(color::Rgb(color[0], color[1], color[2]))) );
            for y in 0..height as usize {
                for x in 0..width as usize {
                    let str = u8_to_string(grid[y][x]);

                    lines[y].push_str(str.as_str());
                }
            }
        }
        c => {
            for y in 0..height as usize {
                let ry: f32 = y as f32 / height as f32;
                let color = c.to_rgb(ry);
                lines[y].push_str(&format!("{}", color::Fg(color::Rgb(color[0], color[1], color[2]))));
                for x in 0..width as usize {
                    if grid[y].len() > x {
                        let str = u8_to_string(grid[y][x]);

                        lines[y].push_str(str.as_str());
                    }
                }
            }        
        }
    }
    lines
}

fn u8_to_string(pixel: GridPixel) -> String {
    let str = match pixel {
        GridPixel::Bar(b) => {
            let pixel= match b {
                // top
                0 => " ",
                8 => "█",
                7 => "▇",
                6 => "▆",
                5 => "▅",
                4 => "▄",
                3 => "▃",
                2 => "▂",
                1 => "▁", 
                // bottom, not the same precision as top
                16 => "█",
                15 => "█",
                14 => "▀",
                13 => "▀",
                12 => "▀",
                11 => "▔",
                10 => "▔",
                9 => "▔",
                _ => " ",   
            };
            pixel.to_string()
        }
        GridPixel::Char(f) => {
            f.to_string()
        }
    };

    str.to_string()
}

pub fn draw(
    grid: Vec<Vec<GridPixel>>, 
    screen: &mut BufWriter<StdoutLock>, 
    size: [u16; 2], 
    color: Color,
    width: u8,
    spacing: u8,
    mirror_x_achsis: bool,
) -> Result<(), Box<dyn Error>> {
    //let calculated_width: u16 = get_bar_number(width, spacing, size[0]) as u16;

    //let grid = gen_grid(size[0], size[1], &data, width, spacing, mirror_x_achsis);
    let lines = get_lines(size[0], size[1], grid, color);

    for y in 0..size[1] as usize {
        write!( screen, "{}", termion::cursor::Goto(0, size[1] - y as u16) )?;
        let line: &str = &lines[y].clone()[..]; // prevents line overflow not implemented rn
        write!( screen, "{}", line)?;
    }

    Ok(())
}