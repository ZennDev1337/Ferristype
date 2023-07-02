use crate::terminalui::color::{bad, good};
use std::fmt::{self, Write};

pub struct Input<'a> {
    line: &'a str,
    line_length: usize,
    cursor: usize,
    input: Vec<char>,
    pub correct: bool,
}

impl<'a> Input<'a> {
    pub fn new(line: &'a str) -> Self {
        let input = Vec::with_capacity(line.len());
        let line_length = line.len();
        Input {
            line,
            line_length,
            cursor: 0,
            input,
            correct: true,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.cursor == self.line_length
    }

    pub fn update(&mut self, input: char) -> bool {
        let correct = match self.line.chars().nth(self.cursor) {
            None => false,
            Some(c) => c == input,
        };
        self.cursor += 1;
        self.input.push(input);
        self.correct &= correct;
        correct
    }

    pub fn delete_one(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.input.pop();
    }
}

impl fmt::Display for Input<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, c) in self.line.chars().enumerate() {
            if i >= self.cursor {
                f.write_char(c)?;
            } else {
                if self.input[i] == c {
                    f.write_str(&good(c))?;
                } else {
                    f.write_str(&bad(c))?;
                }
            }
        }
        if self.cursor > self.line_length {
            for i in self.line_length..self.cursor {
                f.write_str(&bad(self.input[i]))?;
            }
        }

        Ok(())
    }
}
