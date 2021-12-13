use std::io::{StdoutLock, Write};
use std::error::Error;
use termion::color;
use crate::config::Color;
use std::io::BufWriter;
use crate::backends::{gen_grid, get_bar_number, GridPixel};

use audioviz::spectrum::Frequency;

fn get_lines(width: u16, height: u16, grid: Vec<Vec<GridPixel>>, color: Color, w: u8, spacing: u8) -> Vec<String> {
    let mut lines: Vec<String> = vec![String::new(); height as usize];
    //let calculated_width: usize = get_bar_number(w, spacing, width) * 2;

    match color {
        Color::Rgb(color) => {
            lines[0].push_str( &format!("{}", color::Fg(color::Rgb(color[0], color[1], color[2]))) );
            for y in 0..height as usize {
                for x in 0..width as usize {
                    let str = u8_to_string(grid[y][x], w, spacing);

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
                        let str = u8_to_string(grid[y][x], w, spacing);

                        lines[y].push_str(str.as_str());
                    }
                }
            }        
        }
    }
    lines
}

fn u8_to_string(pixel: GridPixel, width: u8, spacing: u8) -> String {
    let str = match pixel {
        GridPixel::Bar(b) => {
            let pixel= match b {
                0 => " ",
                8 => "█",
                7 => "▇",
                6 => "▆",
                5 => "▅",
                4 => "▄",
                3 => "▃",
                2 => "▂",
                1 => "▁",      
                _ => " ",   
            };
            pixel.to_string()
        }
        GridPixel::Char(f) => {
            f.to_string()
        }
    };

    let mut string = str.to_string();

    // width
    match pixel {
        GridPixel::Bar(_) => {
            for _ in 1..width {
                string.push(' ');
            };
        },
        GridPixel::Char(_) => {
            for _ in 1..width {
                string.push(' ');
            };
        },
    }

    // spacing
    for _ in 0..spacing {
        string.push(' ');
    };


    string
}

pub fn draw(
    data: &Vec<Frequency>, 
    screen: &mut BufWriter<StdoutLock>, 
    size: [u16; 2], 
    color: Color,
    width: u8,
    spacing: u8,
) -> Result<(), Box<dyn Error>> {
    let calculated_width: u16 = get_bar_number(width, spacing, size[0]) as u16;

    let grid = gen_grid(calculated_width, size[1], &data);
    let lines = get_lines(calculated_width, size[1], grid, color, width, spacing);

    for y in 0..size[1] as usize {
        write!( screen, "{}", termion::cursor::Goto(0, size[1] - y as u16) )?;
        let line: &str = &lines[y].clone()[..]; // prevents line overflow not implemented rn
        write!( screen, "{}", line)?;
    }

    Ok(())
}