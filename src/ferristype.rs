use crate::lang::Language;
use crate::stats::Timer;
use crate::terminalui::{Input, Terminal};
use itertools::peek_nth;
use std::io::{stdin, stdout, Write};
use termion::{cursor, event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Clone)]
pub enum Mode {
    Words(String, usize),
}

impl Default for Mode {
    fn default() -> Self {
        Self::Words("english".to_string(), 10)
    }
}

fn initialize_lines(words: &Vec<String>, words_per_line: u16) -> Vec<String> {
    let mut resu = vec![String::new()];
    let mut w = words.clone();
    let count = words.len() / words_per_line as usize + 1;
    for _ in 0..count {
        let mut line = vec![];
        for _ in 0..words_per_line {
            if let Some(word) = w.pop() {
                line.push(word);
            }
        }
        resu.push(line.join(" "));
    }
    resu.remove(0);
    if words.len() % words_per_line as usize == 0 {
        resu.remove(count - 1);
    }
    resu
}

pub struct FerrisType {
    mode: Mode,
    lines: Vec<String>,
    timer: Timer,
    terminal: Terminal,
    line_peek: u16,
    words_per_line: u16,
}

impl FerrisType {
    pub fn new(mode: Mode) -> Self {
        let words = match &mode {
            Mode::Words(lang, num) => {
                let language = Language::new(lang.to_string());
                language.get_random(num.to_owned())
            }
        };
        let (w, _) = termion::terminal_size().unwrap();
        let lines = initialize_lines(&words, w / 3 / 5);
        Self {
            mode,
            lines,
            timer: Timer::default(),
            terminal: Terminal::new(stdout().into_raw_mode().unwrap()),
            line_peek: 3,
            words_per_line: w / 3 / 5,
        }
    }

    pub fn words(lang: String, n: usize) -> Self {
        Self::new(Mode::Words(lang, n))
    }

    pub fn line_peek(mut self, n: u16) -> Self {
        self.line_peek = n;
        self
    }
    pub fn words_per_line(mut self, n: u16) -> Self {
        self.words_per_line = n;
        self
    }

    pub fn play(mut self) -> std::io::Result<()> {
        let mut stdin = stdin().keys();
        let mut on_last_word = false;
        self.terminal.initialize_screen()?;
        let l = self.lines.clone();
        let line_peek = self.line_peek;
        let mut lines = peek_nth(l.iter());
        let mut lastline = None;
        'lines: while let Some(current_line) = lines.next() {
            let mut input = Input::new(current_line);
            self.terminal.draw_current_line(current_line)?;
            self.terminal.draw_prev_line(lastline)?;
            self.terminal.clear_current_line()?;
            write!(self.terminal.stdout, "{}", input)?;
            for i in 0..line_peek {
                self.terminal.clear_next_line(i)?;
                let next_line = lines.peek_nth(i.into());
                self.terminal.draw_future_line(next_line, i)?;
                if i == 0 {
                    on_last_word = true;
                }
            }
            lastline = Some(current_line);
            loop {
                write!(
                    self.terminal.stdout,
                    "{}",
                    cursor::Goto(self.terminal.type_width, self.terminal.type_height)
                )?;
                self.terminal.stdout.flush()?;
                let c = stdin.next().unwrap()?;
                self.timer.record_key(c);
                match c {
                    Key::Char('\t') => return FerrisType::new(self.mode.clone()).play(),
                    Key::Char('\n') => {
                        break;
                    }
                    Key::Char(' ') if input.is_complete() => break,
                    Key::Char(c) => {
                        let correct = input.update(c);
                        if correct && on_last_word && input.is_complete() {
                            break;
                        }
                    }
                    Key::Backspace => {
                        self.terminal.clear_current_line()?;
                        input.delete_one();
                    }
                    Key::Ctrl('c') => {
                        break 'lines;
                    }
                    _ => {}
                }
                write!(self.terminal.stdout, "{}", input)?;
                self.terminal.stdout.flush()?;
            }
            self.terminal.clear_current_line()?;
        }
        self.terminal.finalize(self.timer, self.lines)?;
        Ok(())
    }
}
