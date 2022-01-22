use audioviz::audio_capture::capture::{Capture, CaptureReceiver};
use crate::config::Config;
use crate::backends::GridPixel;
use audioviz::spectrum::stream::{Stream, StreamController};

pub enum ConverterType {
    Stream(Stream),
    Capture(Capture),
}

pub struct Converter {
    conv_type: ConverterType,
    raw_buf: Vec<f32>,
    show_vec: Vec<f32>,
    pub raw_receiver: Option<CaptureReceiver>,
    pub stream_controller: Option<StreamController>,
    pub config: Config,
    pub resolution: usize,
}
impl Converter {
    pub fn from_capture(capture: Capture, config: Config) -> Self {
        let raw_receiver = capture.get_receiver().unwrap();
        Self {
            conv_type: ConverterType::Capture(capture),
            raw_buf: Vec::new(),
            show_vec: Vec::new(),
            raw_receiver: Some(raw_receiver),
            stream_controller: None,
            config,
            resolution: 0,
        }
    }

    pub fn from_stream(stream: Stream, config: Config) -> Self {
        let stream_controller = stream.get_controller();
        Self {
            conv_type: ConverterType::Stream(stream),
            raw_buf: Vec::new(),
            show_vec: Vec::new(),
            raw_receiver: None,
            stream_controller: Some(stream_controller),
            config,
            resolution: 0,
        }
    }

    fn get_data(&mut self) -> Option<Vec<f32>> {
        if let Some(raw) = &self.raw_receiver {
            let mut data: Vec<f32> = match raw.receive_data() {
                Some(d) => {
                    let mut b: Vec<f32> = Vec::new();

                    let bufs = d.chunks(1);
                    for buf in bufs {
                        let mut max: f32 = 0.0;
                        for value in buf {
                            let value = value * 30.0 * self.config.audio.processor.volume;
                            if value > max {
                                max = value
                            }
                        }
                        b.push(max)
                    }
                    b
                },
                None => Vec::new()
            };
            self.raw_buf.append(&mut data);
            if self.raw_buf.len() >= self.resolution {
                self.show_vec = self.raw_buf[0..self.resolution].to_vec();
                self.raw_buf.drain(..);
            }
            return Some(self.show_vec.clone());
            /*
            self.raw_vec.append(&mut data);
            if self.raw_vec.len() > self.resolution {
                let to_clear: usize = self.raw_vec.len() - self.resolution;
                self.raw_vec.drain(..to_clear);

                return Some(self.raw_vec.clone());
            }
            */
        }
        if let Some(stream) = &self.stream_controller {
            if let Ok(freqs) = stream.get_frequencies() {
                let data: Vec<f32> = freqs
                    .into_iter()
                    .map(|x| x.volume)
                    .collect();
                return Some(data);
            }
        }
        
        None
    }

    
    pub fn gen_grid(&mut self, x_size: u16, y_size: u16) -> Vec<Vec<GridPixel>> {
        let mut buffer: Vec<Vec<GridPixel>> = vec![vec![GridPixel::Bar(0); x_size as usize]; y_size as usize];

        let mut data = self.get_data().unwrap_or(Vec::new());

        if self.config.mirror {
            for i in 0..data.len() {
                data.insert(0, data[i*2].clone());
            }
        }
        for d in data.iter_mut() {
            *d += y_size as f32 / 8.0;
        }

        let mut screen_x: usize = 0;
        //let mut x: usize = 0;
        for x in 0..x_size as usize {
            if data.len() > x {
                let height: usize = data[x].trunc() as usize;

                // can range from 0 to 1, top of bar for 8 times more precision
                let precision_top: f32 = data[x] - height as f32;
                let precision_bar: u8 = (precision_top * 8.0) as u8 + 1;
                //let precision_bar: u8 = 8;

                if self.config.mirror_x_achsis {
                    for _ in 0..self.config.width {
                        for y in 0..height {
                            if buffer.len() > (y_size as usize / 2 + y + 1) && buffer.len() > (y_size as usize / 2 - y)
                            && buffer[y].len() > screen_x {
                                // top mirror
                                buffer[y_size as usize / 2 + y + 1][screen_x] = GridPixel::Bar(8);
        
                                // bottom mirror
                                buffer[y_size as usize/ 2 - y][screen_x] = GridPixel::Bar(8);
                            }
                        } 

                        // precision bars
                        if buffer.len() > (y_size as usize / 2 + height + 1) && buffer.len() > (y_size as usize / 2 - height)
                        && buffer[height].len() > screen_x {
                            // top
                            buffer[y_size as usize / 2 + height + 1][screen_x] = GridPixel::Bar(precision_bar);

                            // bottom
                            buffer[y_size as usize / 2 - height][screen_x] = GridPixel::Bar(precision_bar + 8 );
                        }
                        screen_x += 1;
                    }
                }
                else {
                    for _ in 0..self.config.width {
                        for y in 0..height {
                            if buffer.len() > y && buffer[y].len() > screen_x {
                                buffer[y][screen_x] = GridPixel::Bar(8);
                            }
                        }
        
                        // precision top bar
                        if buffer.len() > height && buffer[height].len() > screen_x {
                            buffer[height][screen_x] = GridPixel::Bar(precision_bar);
                        }
                        screen_x += 1;
                    }
                }
                screen_x += self.config.spacing as usize;
            }
        }

        buffer
    }

    pub fn set_resolution(&mut self, resolution: usize) {
        self.resolution = resolution;
        if let Some(stream) = &self.stream_controller {
            stream.set_resolution(resolution);
        }
    }
}