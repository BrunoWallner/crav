use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;

#[derive(Debug, Clone)]
pub enum Event<I> {
    Input(I),
    Resize( (u16, u16) ),
    Tick,
}
pub struct EventHandler {
    rx: mpsc::Receiver<Event<Key>>,
    pub tick_rate: Duration,
}
impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::channel();

        let tx_clone = tx.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            for evt in stdin.keys() {
                match evt {
                    Ok(key) => tx_clone.send(Event::Input(key)).unwrap(),
                    Err(_) => (),
                }
            }
        });

        let tx_clone = tx.clone();
        thread::spawn(move || {
            let mut t_size_old = termion::terminal_size().unwrap();
            loop {
                let t_size = termion::terminal_size().unwrap();
                if t_size_old != t_size {
                    tx_clone.send(Event::Resize( (t_size.0, t_size.1) )).unwrap();
                    t_size_old = t_size;
                }
                tx_clone.send(Event::Tick).unwrap();
                thread::sleep(tick_rate);
            }
        });

        EventHandler { rx, tick_rate }
    }

    pub fn get(&self) -> Option<Event<Key>> {
        match self.rx.recv() {
            Ok(e) => Some(e),
            Err(_) => None,
        }
    }
}