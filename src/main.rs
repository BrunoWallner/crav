use std::error::Error;

mod backends;

mod config;
use audioviz::*;

mod audio;

use gag::Gag;

use clap::{Arg, App, AppSettings};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("audiovis")
    .version("0.1.0")
    .author("Luca Biendl <b.lucab1211@gmail.com>")
    .about("tool to visualize audio")
    .setting(AppSettings::ColorAlways)
    .setting(AppSettings::ColoredHelp)

                
    .arg(Arg::with_name("backend")
                .long("backend")
                .takes_value(true)
                .help("can be Termion or Wgpu"))

    .get_matches();

    let backend: backends::Backend = match matches.value_of("backend") {
        Some(b) => match b.to_lowercase().as_str() {
            "termion" => backends::Backend::Termion,
            "wgpu" => backends::Backend::Wgpu,
            _ => panic!("invalid backend")
        }
        None => backends::Backend::Termion,
    };

    let mut config = config::Config::default();

    let audio_config = Config {
        max_frequency: config.max_frequency as usize,
        fft_resolution: config.fft_resolution as usize,
        smoothing_amount: config.smoothing_amount as usize,
        smoothing_size: config.smoothing_size as usize,
        volume: config.volume,
        buffering: config.buffering as usize,
        resolution: 0.1,
        ..Default::default()
    };
    let audio = AudioStream::init(audio_config);
    let audio_ev = audio.get_event_sender();

    // streaming audio using cpal to audiostream
    let audio_ev_clone = audio_ev.clone();
    std::thread::spawn(move || loop {
        let _gag = Gag::stderr().unwrap();
        let _stream = audio::stream_audio(audio_ev_clone.clone(), audio::AudioDevice::Output(0));
        std::thread::park();
    });

    let color_modes: Vec<config::Color> = vec![
        config::Color::Rgb([0, 255, 0]),
        config::Color::Rgb([0, 0, 255]),
        config::Color::Rgb([255, 0, 50]),
        config::Color::Rgb([127, 0, 255]),
        config::Color::Rgb([255, 255, 0]),
        config::Color::Rgb([0, 255, 255]),
        config::Color::Rgb([255, 0, 255]),
        config::Color::Rgb([255, 0, 0]),
        config::Color::Rainbow,
    ];

    backend.run(&mut config, audio, color_modes);


    Ok(())
}
