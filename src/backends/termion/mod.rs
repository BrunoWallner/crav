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

use audioviz::spectrum::stream::StreamController;

use crate::backends::audio_to_grid::Converter;

pub fn run(mut config: &mut Config, mut converter: Converter) {
    let raw = stdout().into_raw_mode().unwrap();
    let mut screen = AlternateScreen::from(raw);
    write!(screen, "{}", termion::cursor::Hide).unwrap();
    write!(screen, "{}", termion::clear::All).unwrap();

    let ev = events::EventHandler::new(Duration::from_millis(1000 / config.fps));


    let (mut width, mut height) = termion::terminal_size().unwrap();

    let mut bar_number: usize = get_bar_number(config.width, config.spacing, width);
    if config.mirror {bar_number /= 2}
    converter.set_resolution(bar_number);

    'main: loop {
        let mut screen = BufWriter::new(screen.lock());

        let grid = converter.gen_grid(width, height);
        bars::draw(
            grid, 
            &mut screen, 
            [width, height], 
            config.color.clone(), 
            config.width, 
            config.spacing, 
            config.mirror_x_achsis
        ).unwrap();

        screen.flush().unwrap();

        match ev.get().unwrap() {
            events::Event::Input(input) => match input {
                Key::Char('q') => break 'main,
                _ => (),
            }
            events::Event::Resize( (w, h)) => {
                width = w;
                height = h;

                let mut bar_number: usize = get_bar_number(config.width, config.spacing, w);
                if config.mirror {bar_number /= 2}
                converter.set_resolution(bar_number);
                write!(screen, "{}", termion::clear::All).unwrap();
            }
            _ => (),
        }
    }
}