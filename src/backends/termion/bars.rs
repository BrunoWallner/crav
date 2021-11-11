use std::io::{StdoutLock, Write};
use std::error::Error;
use termion::color;
use crate::config::Color;
use std::io::BufWriter;
use crate::backends::{gen_grid, get_bar_number};

fn get_lines(width: u16, height: u16, grid: Vec<Vec<u8>>, color: Color, w: u8, spacing: u8) -> Vec<String> {
    let mut lines: Vec<String> = vec![String::new(); height as usize];
    let calculated_width: usize = get_bar_number(w, spacing, width) * 2;

    match color {
        Color::Rgb(color) => {
            lines[0].push_str( &format!("{}", color::Fg(color::Rgb(color[0], color[1], color[2]))) );
            for y in 0..height as usize {
                for x in 0..calculated_width {
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
                for x in 0..calculated_width {
                    let str = u8_to_string(grid[y][x], w, spacing);

                    lines[y].push_str(str.as_str());
                }
            }        
        }
    }
    lines
}

fn u8_to_string(u8: u8, width: u8, spacing: u8) -> String {
    let str = match u8 {
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

    let mut string = str.to_string();

    // width
    for _ in 1..width {
        string.push_str(&str);
    };

    // spacing
    for _ in 0..spacing {
        string.push_str(" ");
    };

    string
}

pub fn draw(
    data: &Vec<f32>, 
    screen: &mut BufWriter<StdoutLock>, 
    mut size: [u16; 2], 
    color: Color,
    width: u8,
    spacing: u8,
) -> Result<(), Box<dyn Error>> {
    size[0] /= 2; // because bar_width = 2;

    let grid = gen_grid(size[0], size[1], &data);
    let lines = get_lines(size[0], size[1], grid, color, width, spacing);

    for y in 0..size[1] as usize {
        write!( screen, "{}", termion::cursor::Goto(0, size[1] - y as u16) )?;
        let line: &str = &lines[y].clone()[..]; // prevents line overflow not implemented rn
        write!( screen, "{}", line)?;
    }

    Ok(())

}