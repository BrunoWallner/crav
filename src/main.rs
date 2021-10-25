use std::error::Error;

mod backends;

mod config;
use audioviz::*;

mod audio;

use gag::Gag;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::default();

    let audio_config = Config {
        max_frequency: config.max_frequency as usize,
        fft_resolution: config.fft_resolution as usize,
        smoothing_amount: config.smoothing_amount as usize,
        smoothing_size: config.smoothing_size as usize,
        volume: config.volume,
        buffering: config.buffering as usize,
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
    //let mut color_modes = color_modes.iter().cycle();

    let backend = backends::Backend::Wgpu;
    backend.run(&mut config, audio, color_modes);


    Ok(())
}
