use std::error::Error;

mod backends;

mod config;
pub use audioviz::*;

mod audio;

#[allow(unused_imports)]
use gag::Gag;

use clap::{Arg, App, AppSettings};

use std::fs;

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

    .arg(Arg::with_name("print_config")
                .short("p")
                .long("print_config")
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

    let audio = AudioStream::init(config.audio.clone());
    let audio_controller = audio.get_controller();

    // streaming audio using cpal to audiostream
    let a_c = audio_controller.clone();
    std::thread::spawn(move || loop {
        //let _gag = Gag::stderr().unwrap();
        let _stream = audio::stream_audio(a_c.clone(), audio::AudioDevice::Output(0));
        std::thread::park();
    });


    backend.run(&mut config, audio_controller);

    println!("bye bye!");


    Ok(())
}
