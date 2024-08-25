use crossterm::ExecutableCommand;
use std::io::Write;

use crossterm::cursor::MoveTo;
use crossterm::terminal;

use crate::command::Command;
use crate::database::{self, Database};
use crate::home::Home;
use crate::spreadsheet::{self, Spreadsheet};

pub struct AppState {
    pub database: Database,
    pub spreadsheet: Spreadsheet,
    pub command: Command,
    pub home: Home,
    pub mode: AppMode,
}

impl AppState {
    pub fn new(cell_width: usize, cell_height: usize) -> Self {
        let spreadsheet = Spreadsheet::new(cell_width, cell_height);
        let command = Command::new();
        let mut database = Database::new().unwrap();
        let sheets = database.get_sheets().unwrap();
        let home = Home::new(sheets);
        Self {
            home,
            database,
            spreadsheet,
            command,
            mode: AppMode::Normal,
        }
    }
    pub fn clear_screen<W: Write>(&self, stdout: &mut W) {
        let (width, height) = terminal::size().unwrap();
        stdout.execute(MoveTo(0, height)).unwrap();
        stdout
            .execute(terminal::Clear(terminal::ClearType::FromCursorUp))
            .unwrap();
    }

    pub fn handle_search(&self) {
        println!("Search")
    }

    pub fn open_sheet<W: Write>(&mut self, out: &mut W) {
        self.mode = AppMode::Normal;
        let selected = &self.home.sheets[self.home.selected];
        self.spreadsheet.id = selected.name.to_string();
        self.clear_screen(out);
    }

    pub fn handle_command(&mut self) {
        let command: &str = &self.command.input;

        match command {
            ":w" => {
                let _result = self
                    .database
                    .write_all_cells(&self.spreadsheet.id, self.spreadsheet.cells.clone())
                    .unwrap();
            }
            _ => {}
        }
    }
}
#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Command,
    Home,
}
