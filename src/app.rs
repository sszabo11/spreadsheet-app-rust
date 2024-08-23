use crate::command::Command;
use crate::spreadsheet::{self, Spreadsheet};

pub struct AppState {
    pub spreadsheet: Spreadsheet,
    pub command: Command,
    pub mode: AppMode,
}

impl AppState {
    pub fn new(cell_width: usize, cell_height: usize) -> Self {
        let spreadsheet = Spreadsheet::new(cell_width, cell_height);
        let command = Command::new();

        Self {
            spreadsheet,
            command,
            mode: AppMode::Normal,
        }
    }
}
#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Command,
}
