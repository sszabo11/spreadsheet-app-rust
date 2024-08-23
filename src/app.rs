use crate::command::Command;
use crate::database::{self, Database};
use crate::spreadsheet::{self, Spreadsheet};

pub struct AppState {
    pub database: Database,
    pub spreadsheet: Spreadsheet,
    pub command: Command,
    pub mode: AppMode,
}

impl AppState {
    pub fn new(cell_width: usize, cell_height: usize) -> Self {
        let spreadsheet = Spreadsheet::new(cell_width, cell_height);
        let command = Command::new();
        let database = Database::new().unwrap();
        Self {
            database,
            spreadsheet,
            command,
            mode: AppMode::Normal,
        }
    }

    pub fn handle_search(&self) {
        println!("Search")
    }

    pub fn handle_command(&mut self) {
        let command: &str = &self.command.input;

        match command {
            ":w" => {
                let _result = self
                    .database
                    .write_all_cells("1", self.spreadsheet.cells.clone())
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
}
