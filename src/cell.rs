use std::io::Write;

use crossterm::style::Color;

use crate::spreadsheet::{self, Spreadsheet};

pub struct ActiveCell {
    pub row: usize,
    pub col: usize,
}

impl ActiveCell {
    pub fn set(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn move_up<W: Write>(&mut self, out: &mut W, spreadsheet: Spreadsheet) {
        spreadsheet.clear_prev(out);
        self.row -= 1;
    }

    pub fn move_down<W: Write>(&mut self, out: &mut W, spreadsheet: Spreadsheet) {
        spreadsheet.clear_prev(out);
        self.row -= 1;
    }
    pub fn move_right<W: Write>(&mut self, out: &mut W, spreadsheet: Spreadsheet) {
        spreadsheet.clear_prev(out);
        self.row -= 1;
    }
    pub fn move_left<W: Write>(&mut self, out: &mut W, spreadsheet: Spreadsheet) {
        spreadsheet.clear_prev(out);
        self.row -= 1;
    }
}

#[derive(Clone)]
pub struct Cell {
    pub value: String,
    pub color: Color,
    pub formula: bool,
}

// impl Cell {
//     fn formula(&self, )
// }
