mod bars;
mod events;

use crate::config::Config;
use crate::config::Color;
use crate::config::Width;

use std::{io::BufWriter};
use termion::event::Key;
use std::error::Error;

use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Write, stdout};

pub fn run(mut config: &mut Config, audio: audioviz::AudioStream, color_modes: Vec<Color>) -> Result<(), Box<dyn Error>> {
    let mut color_modes = color_modes.iter().cycle();

    let raw = stdout().into_raw_mode()?;
    let mut screen = AlternateScreen::from(raw);
    write!(screen, "{}", termion::cursor::Hide)?;
    write!(screen, "{}", termion::clear::All)?;

    let ev = events::EventHandler::new(config.tick_rate);


    let (mut width, mut height) = termion::terminal_size().unwrap();
    audio.set_bar_number( (width as f32 * 0.25) as usize );
    'main: loop {
        let mut data = audio.get_audio_data();
        for i in 0..data.len() {
            data.insert(0, data[i * 2]);
        }

        let mut screen = BufWriter::new(screen.lock());

        bars::draw(&data, &mut screen, [width, height], config.color, config.width)?;

        screen.flush()?;

        match ev.get().unwrap() {
            events::Event::Input(input) => match input {
                Key::Char('q') => break 'main,
                Key::Char('c') => config.color = *color_modes.next().unwrap(),
                Key::Char('+') => audio.adjust_volume(0.1),
                Key::Char('-') => audio.adjust_volume(-0.1),
                Key::Char('w') => {
                    config.width = match config.width {
                        Width::Full => Width::Half,
                        Width::Half => Width::Full, 
                    }
                }
                _ => (),
            }
            events::Event::Resize( (w, h)) => {
                audio.set_bar_number( (w as f32 * 0.25) as usize);
                write!(screen, "{}", termion::clear::All)?;

                width = w;
                height = h;
            }
            _ => (),
        }
    }

    Ok(())
}