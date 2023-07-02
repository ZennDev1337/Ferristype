use std::time::{Duration, Instant};
use termion::event::Key;

#[derive(Default, Debug)]
pub struct Timer {
    pub timer: Option<Instant>,
    pub key_timer: Vec<(Duration, Key)>,
    pub word_list: Vec<String>,
}

impl Timer {
    pub fn record_key(&mut self, k: Key) {
        if let None = self.timer {
            self.timer = Some(Instant::now());
        }
        let t = self.timer.unwrap().elapsed();
        self.key_timer.push((t, k));
    }
}
