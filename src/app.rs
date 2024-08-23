use crate::spreadsheet::{self, Spreadsheet};

pub struct AppState {
    pub spreadsheet: Spreadsheet,
    pub mode: AppMode,
}

impl AppState {
    pub fn new(cell_width: usize, cell_height: usize) -> Self {
        let spreadsheet = Spreadsheet::new(cell_width, cell_height);
        Self {
            spreadsheet,
            mode: AppMode::Normal,
        }
    }
}

pub enum AppMode {
    Normal,
    Command,
}
