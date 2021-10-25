use std::io::{StdoutLock, Write};
use std::error::Error;
use termion::color;
use crate::config::Color;
use crate::config::Width;
use std::io::BufWriter;
use crate::backends::gen_grid;

fn get_lines(width: u16, height: u16, grid: Vec<Vec<u8>>, color: Color, w: Width) -> Vec<String> {
    let mut lines: Vec<String> = vec![String::new(); height as usize];
    match color {
        Color::Rgb(color) => {
            lines[0].push_str( &format!("{}", color::Fg(color::Rgb(color[0], color[1], color[2]))) );
            for y in 0..height as usize {
                for x in 0..width as usize {
                    let str = match grid[y][x] {
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
                    let mut str = str.to_string();
                    match w {
                        Width::Half =>  {
                            str.push_str(" ");
                        }
                        Width::Full => {
                            let ch: char = str.chars().collect::<Vec<char>>()[0];
                            str.push(ch);           
                        }
                    }
                    lines[y].push_str(str.as_str());
                }
            }
        }
        Color::Rainbow => {
            for y in 0..height as usize {
                let ry: f32 = y as f32 / height as f32;
                let color = Color::rainbow_from_y(ry);
                lines[y].push_str(&format!("{}", color::Fg(color::Rgb(color[0], color[1], color[2]))));
                for x in 0..width as usize {
                    let str = match grid[y][x] {
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
                    let mut str = str.to_string();
                    match w {
                        Width::Half =>  {
                            str.push_str(" ");
                        }
                        Width::Full => {
                            let ch: char = str.chars().collect::<Vec<char>>()[0];
                            str.push(ch);           
                        }
                    }

                    lines[y].push_str(str.as_str());
                }
            }        
        }
    }
    lines
}

pub fn draw(
    data: &Vec<f32>, 
    screen: &mut BufWriter<StdoutLock>, 
    mut size: [u16; 2], 
    color: Color,
    width: Width,
) -> Result<(), Box<dyn Error>> {
    size[0] /= 2; // because bar_width = 2;

    let grid = gen_grid(size[0], size[1], &data);
    let lines = get_lines(size[0], size[1], grid, color, width);

    for y in 0..size[1] as usize {
        write!( screen, "{}", termion::cursor::Goto(0, size[1] - y as u16) )?;
        write!( screen, "{}", lines[y] )?;
    }

    Ok(())

}