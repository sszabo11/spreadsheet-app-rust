use std::io::{stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::style::Stylize;
use crossterm::terminal::Clear;
use crossterm::ExecutableCommand;

use crossterm::{event::KeyCode, execute, terminal};

pub struct Home {
    pub sheets: Vec<Sheet>,
    pub selected: usize,
}

impl Home {
    pub fn new(sheets: Vec<Sheet>) -> Self {
        Self {
            sheets,
            selected: 0,
        }
    }

    pub fn focus<W: Write>(&mut self, stdout: &mut W, key: KeyCode) {
        self.handle_key(key, stdout);
    }

    pub fn draw<W: Write>(&self, out: &mut W) {
        self.draw_title(out);
        let width = 40;
        let height = 10;
        let terminal = terminal::size().unwrap();
        out.execute(MoveTo(
            terminal.0 / 2 - width / 2,
            terminal.1 / 2 - height / 2,
        ))
        .unwrap();

        let horizontal_line = "-".repeat(width as usize - 2);
        print!("+{}+", horizontal_line);

        for line in 1..height {
            out.execute(MoveTo(
                terminal.0 / 2 - width / 2,
                terminal.1 / 2 - height / 2 + line,
            ))
            .unwrap();
            print!("|{}|", " ".repeat(width as usize - 2));
        }
        out.execute(MoveTo(
            terminal.0 / 2 - width / 2,
            terminal.1 / 2 - height / 2 + height,
        ))
        .unwrap();
        print!("+{}+", horizontal_line);

        out.execute(Clear(terminal::ClearType::UntilNewLine))
            .unwrap();
        for (i, sheet) in self.sheets.iter().enumerate() {
            out.execute(MoveTo(
                terminal.0 / 2 - width / 2 + 3,
                terminal.1 / 2 - height / 2 + 2 + i as u16,
            ))
            .unwrap();
            if self.selected == i {
                print!("{}", sheet.name.clone().on_grey().black());
                println!("{}<", " ".repeat(width as usize - 6 - sheet.name.len()));
            } else {
                println!("{}", sheet.name);
            }
        }
    }

    pub fn draw_title<W: Write>(&self, out: &mut W) {
        let title = format!(
            "{}",
            r#"
 ______          _         _____ _               _       
 | ___ \        | |       /  ___| |             | |      
 | |_/ /   _ ___| |_ _   _\ `--.| |__   ___  ___| |_ ___ 
 |    / | | / __| __| | | |`--. \ '_ \ / _ \/ _ \ __/ __|
 | |\ \ |_| \__ \ |_| |_| /\__/ / | | |  __/  __/ |_\__ \
 \_| \_\__,_|___/\__|\__, \____/|_| |_|\___|\___|\__|___/
                     __/ |                              
                    |___/                                                                              
    "#
        );
        let terminal = terminal::size().unwrap();
        out.execute(MoveTo(0, terminal.1 / 2 - 3)).unwrap();
        print!("{}", title)
    }

    pub fn handle_key<W: Write>(&mut self, key: KeyCode, out: &mut W) {
        match key {
            KeyCode::Up => {
                if self.selected == 0 {
                    self.selected = self.sheets.len() - 1
                } else {
                    self.selected -= 1
                }

                self.draw(out);
            }
            KeyCode::Down => {
                if self.selected == self.sheets.len() - 1 {
                    self.selected = 0
                } else {
                    self.selected += 1
                }
                self.draw(out);
            }

            KeyCode::Enter => {}
            _ => {}
        }
    }
}

pub struct Sheet {
    pub name: String,
}
