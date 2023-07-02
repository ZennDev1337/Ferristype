use crate::stats::Timer;
use crate::terminalui::Input;
use std::{fmt, time::Duration};
use termion::{clear, cursor, event::Key};

pub struct Statistics {
    wpm: f64,
    raw_wpm: f64,
    accuracy: f64,
    time: Duration,
    correct_lines: usize,
    incorrect_lines: usize,
}

impl Statistics {
    pub fn calculate(timer: Timer, lines: &Vec<String>) -> Option<Statistics> {
        let mut correct_lines = 0;
        let mut incorrect_lines = 0;
        let mut correct_chars = 0;
        let mut incorrect_chars = 0;
        let mut inputs = lines.into_iter().map(|x| Input::new(x)).peekable();
        let mut current_input = inputs.next().unwrap();
        let mut final_time = Duration::ZERO;
        for (t, k) in timer.key_timer {
            final_time = t;
            match k {
                Key::Char('\n') => {
                    if current_input.is_complete() && current_input.correct {
                        correct_lines += 1;
                    } else {
                        incorrect_lines += 1;
                    }
                    match inputs.next() {
                        Some(n) => current_input = n,
                        None => break,
                    }
                }
                Key::Char(c) => {
                    let correct = current_input.update(c);
                    if correct {
                        correct_chars += 1;
                    } else {
                        incorrect_chars += 1;
                    }
                    if current_input.is_complete()
                        && current_input.correct
                        && inputs.peek().is_none()
                    {
                        correct_lines += 1
                    }
                }
                Key::Backspace => {
                    current_input.delete_one();
                }
                Key::Ctrl('c') => break,
                _ => unreachable!("Unexpected key in history"),
            }
        }
        let wpm = correct_chars as f64 / final_time.as_millis() as f64 * 12000.0;
        let raw_wpm =
            (correct_chars + incorrect_chars) as f64 / final_time.as_millis() as f64 * 12000.0;
        let accuracy = correct_chars as f64 / (correct_chars + incorrect_chars) as f64;
        Some(Statistics {
            wpm,
            raw_wpm,
            accuracy,
            time: final_time,
            correct_lines,
            incorrect_lines,
        })
    }
}

impl fmt::Display for Statistics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", cursor::Goto(1, 1), clear::CurrentLine)?;
        write!(
            f,
            "T: {}s, lines without failure: {}, lines without correcting: {}",
            self.time.as_millis() as f64 / 1000.0,
            self.correct_lines,
            self.incorrect_lines,
        )?;
        write!(f, "{}{}", cursor::Goto(1, 2), clear::CurrentLine)?;
        write!(
            f,
            "WPM: {:.2}, Raw: {:.2}, Accuracy {:.3}",
            self.wpm,
            self.raw_wpm,
            self.accuracy * 100.0
        )
    }
}
