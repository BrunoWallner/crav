mod bars;
mod events;

use crate::backends::get_bar_number;
use crate::config::Config;

use std::{io::BufWriter};
use std::time::Duration;
use termion::event::Key;

use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Write, stdout};

pub fn run(mut config: &mut Config, audio_controller: audioviz::AudioStreamController) {
    let raw = stdout().into_raw_mode().unwrap();
    let mut screen = AlternateScreen::from(raw);
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    write!(screen, "{}", termion::clear::All).unwrap();

    let ev = events::EventHandler::new(Duration::from_millis(1000 / config.fps));


    let (mut width, mut height) = termion::terminal_size().unwrap();

    let mut bar_number: usize = get_bar_number(config.width, config.spacing, width);
    if config.mirror {bar_number /= 2}
    audio_controller.set_resolution(bar_number);

    'main: loop {
        let mut data = audio_controller.get_frequencies();
        if config.mirror {
            for i in 0..data.len() {
                data.insert(0, data[i * 2].clone());
            }
        }

        let mut screen = BufWriter::new(screen.lock());

        bars::draw(&data, &mut screen, [width, height], config.color.clone(), config.width, config.spacing, config.show_freqs).unwrap();

        screen.flush().unwrap();

        match ev.get().unwrap() {
            events::Event::Input(input) => match input {
                Key::Char('q') => break 'main,
                Key::Char('+') => audio_controller.adjust_volume(1.1),
                Key::Char('-') => audio_controller.adjust_volume(0.9),
                Key::Char('m') => {
                    let mut bar_number = get_bar_number(config.width, config.spacing, width);
                    config.mirror = !config.mirror;
                    if config.mirror {
                        bar_number /= 2
                    }
                    audio_controller.set_resolution(bar_number);
                },

                _ => (),
            }
            events::Event::Resize( (w, h)) => {
                width = w;
                height = h;

                let mut bar_number: usize = get_bar_number(config.width, config.spacing, w);
                if config.mirror {bar_number /= 2}
                audio_controller.set_resolution(bar_number);
                write!(screen, "{}", termion::clear::All).unwrap();
            }
            _ => (),
        }
    }
}