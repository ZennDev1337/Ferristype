use crate::stats::{Statistics, Timer};
use crate::terminalui::color::subtle;
use std::io::{Stdout, Write};
use termion::{clear, cursor, raw::RawTerminal};

pub struct Terminal {
    pub type_width: u16,
    pub type_height: u16,
    pub stdout: RawTerminal<Stdout>,
}

impl Terminal {
    pub fn new(standard_out: RawTerminal<Stdout>) -> Self {
        let (terminal_width, terminal_height) = termion::terminal_size().unwrap();
        let type_width = terminal_width / 3;
        let type_height = terminal_height / 3;
        Terminal {
            type_width,
            type_height,
            stdout: standard_out,
        }
    }

    pub fn draw_prev_line(&mut self, lastline: Option<&String>) -> std::io::Result<()> {
        if let Some(line) = lastline {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(self.type_width, self.type_height - 1 as u16),
                subtle(line)
            )?;
        }
        self.stdout.flush()
    }

    pub fn draw_current_line(&mut self, current_line: &String) -> std::io::Result<()> {
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(self.type_width, self.type_height),
            subtle(current_line)
        )?;
        self.stdout.flush()
    }

    pub fn draw_future_line(
        &mut self,
        future_line: Option<&&String>,
        i: u16,
    ) -> std::io::Result<()> {
        if let Some(next) = future_line {
            write!(
                self.stdout,
                "{}{}",
                cursor::Goto(self.type_width, self.type_height + i + 1),
                subtle(next)
            )?;
        }
        self.stdout.flush()
    }

    pub fn initialize_screen(&mut self) -> std::io::Result<()> {
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            cursor::Goto(self.type_width, self.type_height),
            cursor::Hide
        )?;
        self.stdout.flush()
    }

    pub fn clear_current_line(&mut self) -> std::io::Result<()> {
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(self.type_width, self.type_height),
            clear::CurrentLine,
        )?;
        self.stdout.flush()
    }

    pub fn clear_next_line(&mut self, i: u16) -> std::io::Result<()> {
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(self.type_width, self.type_height + i + 1),
            clear::CurrentLine,
        )?;
        self.stdout.flush()
    }

    pub fn finalize(mut self, timer: Timer, lines: Vec<String>) -> std::io::Result<()> {
        let statistics = Statistics::calculate(timer, &lines);
        if let Some(statistics) = statistics {
            write!(self.stdout, "{}{}", clear::All, statistics)?;
        }
        cursor::Goto(1, 1);
        write!(self.stdout, "\r\n{}", cursor::Show)?;

        self.stdout.flush()
    }
}
