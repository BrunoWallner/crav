mod bars;
mod events;

use crate::config::Config;
use crate::config::Color;
use crate::config::Width;

use std::{io::BufWriter};
use std::time::Duration;
use termion::event::Key;
use std::error::Error;

use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Write, stdout};

pub fn run(mut config: &mut Config, audio: audioviz::AudioStream, color_modes: Vec<Color>) -> Result<(), Box<dyn Error>> {
    let cm = color_modes.into_iter();
    let mut color_modes = cm.cycle();

    let raw = stdout().into_raw_mode()?;
    let mut screen = AlternateScreen::from(raw);
    write!(screen, "{}", termion::cursor::Hide)?;
    write!(screen, "{}", termion::clear::All)?;

    let ev = events::EventHandler::new(Duration::from_millis(1000 / config.fps));


    let (mut width, mut height) = termion::terminal_size().unwrap();
    let mut bar_number =  (width as f32 * 0.5) as usize;
    if config.mirror {bar_number /= 2}
    audio.set_bar_number(bar_number);
    'main: loop {
        let mut data = audio.get_audio_data();
        if config.mirror {
            for i in 0..data.len() {
                data.insert(0, data[i * 2]);
            }
        }

        let mut screen = BufWriter::new(screen.lock());

        bars::draw(&data, &mut screen, [width, height], config.color.clone(), config.width)?;

        screen.flush()?;

        match ev.get().unwrap() {
            events::Event::Input(input) => match input {
                Key::Char('q') => break 'main,
                Key::Char('c') => config.color = color_modes.next().unwrap(),
                Key::Char('+') => audio.adjust_volume(1.1),
                Key::Char('-') => audio.adjust_volume(0.9),
                Key::Char('m') => {
                    config.mirror = !config.mirror;
                    let mut bar_number =  (width as f32 * 0.5) as usize;
                    if config.mirror {bar_number /= 2}
                    audio.set_bar_number(bar_number);
                },
                Key::Char('w') => {
                    config.width = match config.width {
                        Width::Full => Width::Half,
                        Width::Half => Width::Full, 
                    }
                },
                Key::Char('b') => {
                    let cfg = audio.get_config();
                    if cfg.buffering >= 2 {
                        let config = audioviz::Config {
                            buffering: cfg.buffering - 1,
                            ..cfg
                        };
                        audio.set_config(config);
                    }
                },
                Key::Char('B') => {
                    let cfg = audio.get_config();
                    let config = audioviz::Config {
                        buffering: cfg.buffering + 1,
                        ..cfg
                    };
                    audio.set_config(config);
                },

                _ => (),
            }
            events::Event::Resize( (w, h)) => {
                let mut bar_number =  (w as f32 * 0.5) as usize;
                if config.mirror {bar_number /= 2}
                audio.set_bar_number(bar_number);
                write!(screen, "{}", termion::clear::All)?;

                width = w;
                height = h;
            }
            _ => (),
        }
    }

    Ok(())
}