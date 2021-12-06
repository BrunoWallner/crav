use std::error::Error;

mod backends;
mod config;
pub use audioviz::*;
mod audio;

#[allow(unused_imports)]
use gag::Gag;

use clap::{Arg, App, AppSettings};
use std::fs;

use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    let matches = App::new("audiovis")
    .version("0.1.0")
    .author("Luca Biendl <b.lucab1211@gmail.com>")
    .about("tool to visualize audio")
    .setting(AppSettings::ColorAlways)
    .setting(AppSettings::ColoredHelp)

                
    .arg(Arg::with_name("backend")
                .short("b")
                .long("backend")
                .takes_value(true)
                .help("can be Termion or Wgpu"))

    .arg(Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("path of config"))

    .arg(Arg::with_name("debug_white_noise")
                .long("debug-white-noise")
                .takes_value(false)
                .help("will display white noise instead of captured audio, for debug purposes"))

    .arg(Arg::with_name("print_config")
                .short("p")
                .long("print-config")
                .takes_value(false)
                .help("prints default config to './default_config.json'"))

    .get_matches();

    let backend: backends::Backend = match matches.value_of("backend") {
        Some(b) => match b.to_lowercase().as_str() {
            "terminal" | "t" | "term" => backends::Backend::Terminal,
            "wgpu" | "w" => backends::Backend::Wgpu,
            _ => panic!("invalid backend")
        }
        None => backends::Backend::Terminal,
    };

    if matches.is_present("print_config") {
        let config = config::Config::default();
        let c_str = serde_json::to_string_pretty(&config).unwrap();
        println!("{}", c_str);
        fs::write("./default_config.json", c_str.as_bytes()).unwrap();
        std::process::exit(0);
    }

    //let mut config = config::Config::default();

    let mut config: config::Config = match matches.value_of("config") {
        Some(p) => {
            let c_str = match fs::read(p) {
                Ok(c) => c,
                Err(e) => {
                    println!("{}", e);
                    std::process::exit(1);
                }
            };
            match serde_json::from_slice(&c_str[..]) {
                Ok(c) => c,
                Err(e) => {
                    println!("invalid config: {}", e);
                    std::process::exit(1);
                }
            }
        }
        None =>  {
            config::Config::default()
        }
    };

    let audio = audioviz::spectralizer::stream::Stream::init(config.audio.clone());
    let audio_controller = audio.get_controller();

    // streaming audio using cpal to audiostream or white-noise
    let a_c = audio_controller.clone();
    if matches.is_present("debug_white_noise") {
        std::thread::spawn(move || loop {
            loop {
                let mut buf: Vec<f32> = Vec::new();
                for _ in 0..=255 {
                    let num: f32 = rand::thread_rng().gen();
                    buf.push(num);
                }
                a_c.send_raw_data(&buf);
            }
        });
    } else {
        std::thread::spawn(move || loop {
            //let _gag = Gag::stderr().unwrap();
            let _stream = audio::stream_audio(a_c.clone(), audio::AudioDevice::Output(0));
            std::thread::park();
        });
    }


    backend.run(&mut config, audio_controller);
    Ok(())
}
