use std::io::{stdout, Write};

use crossterm::{
    cursor::{self, MoveTo, Show},
    event::KeyCode,
    execute,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};

pub struct Command {
    pub input: String,
    pub cursor_pos: usize,
}

impl Command {
    pub fn new() -> Self {
        Self {
            cursor_pos: 1,
            input: "".to_string(),
        }
    }
    pub fn focus<W: Write>(&mut self, stdout: &mut W, key: KeyCode) {
        self.write_text(key, stdout);
        self.draw(stdout);
    }

    pub fn draw<W: Write>(&self, out: &mut W) {
        let (_width, height) = terminal::size().unwrap();

        out.execute(cursor::MoveTo(0, height)).unwrap();
        out.execute(Clear(ClearType::CurrentLine)).unwrap();

        print!("{}", self.input);
        out.flush().unwrap();
    }

    pub fn write_text<W: Write>(&mut self, key: crossterm::event::KeyCode, out: &mut W) {
        match key {
            KeyCode::Char(c) => {
                self.input.insert(self.cursor_pos, c);

                self.cursor_pos += 1;

                let input = self.input.clone();

                if input.starts_with(":") {
                    // Command
                    //println!("{}", input);
                }
            }

            KeyCode::Backspace => {
                self.cursor_pos -= 1;
                self.input.remove(self.cursor_pos);
                if self.cursor_pos == 0 {
                    self.cursor_pos = 1
                }
            }

            KeyCode::Left => {
                if self.cursor_pos != 0 {
                    self.cursor_pos -= 1;
                }
            }

            KeyCode::Right => {
                let cell_len = self.input.len();
                if self.cursor_pos != cell_len {
                    self.cursor_pos += 1;
                }
            }

            _ => (),
        }
        //self.draw(out);
    }
}
