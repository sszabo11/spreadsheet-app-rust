use std::{io::Write, usize};

use crossterm::{
    cursor::MoveTo,
    event::{KeyCode, ModifierKeyCode},
    style::Color,
    style::Stylize,
    terminal::{self, Clear, ClearType},
    ExecutableCommand,
};

use crate::{
    cell::{self, Cell},
    formulas::{FormulaHandler, FormulaType},
};

pub struct Spreadsheet {
    pub cells: Vec<Vec<cell::Cell>>,
    pub active_cell: cell::ActiveCell,
    pub cell_width: usize,
    pub cell_height: usize,
    pub text_edit: bool,
    pub fill: bool,
    pub cursor_pos: usize,
}

const AXIS_WIDTH: u16 = 5;
const AXIS_HEIGHT: u16 = 2;

impl Spreadsheet {
    pub fn new(
        fill: bool,
        mut rows: usize,
        mut cols: usize,
        cell_width: usize,
        cell_height: usize,
    ) -> Self {
        let (width, height) = terminal::size().unwrap();

        if fill {
            rows = ((height - AXIS_HEIGHT) / cell_height as u16) as usize;
            cols = ((width - AXIS_WIDTH) / cell_width as u16) as usize;
        }

        let cells = vec![
            vec![
                cell::Cell {
                    value: "".to_string(),
                    color: Color::Red,
                    formula: false,
                };
                cols
            ];
            rows
        ];

        Self {
            cells,
            active_cell: cell::ActiveCell::set(0, 0),
            cell_width,
            cell_height,
            text_edit: false,
            fill,
            cursor_pos: 0,
        }
    }

    pub fn clear_prev<W: Write>(&self, out: &mut W) {
        for line in 1..self.cell_height {
            out.execute(MoveTo(
                self.active_cell.col as u16 * self.cell_width as u16,
                self.active_cell.row as u16 * self.cell_height as u16 + line as u16 + AXIS_HEIGHT,
            ))
            .unwrap();
            out.execute(Clear(ClearType::UntilNewLine)).unwrap();
        }
    }
    pub fn handle_key_press<W: Write>(&mut self, key: KeyCode, out: &mut W) {
        // out.execute(terminal::Clear(ClearType::All)).unwrap();
        //
        if !self.text_edit {
            match key {
                KeyCode::Up => {
                    if self.active_cell.row > 0 {
                        self.clear_prev(out);
                        self.active_cell.row -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.active_cell.row < self.cells.len() - 1 {
                        self.clear_prev(out);
                        self.active_cell.row += 1;
                    }
                }
                KeyCode::Left => {
                    if self.active_cell.col > 0 {
                        self.clear_prev(out);
                        self.active_cell.col -= 1;
                    }
                }
                KeyCode::Right => {
                    if self.active_cell.col < self.cells[0].len() - 1 {
                        self.clear_prev(out);
                        self.active_cell.col += 1;
                    }
                }
                KeyCode::Enter => {
                    let cell_len = self.cells[self.active_cell.row][self.active_cell.col]
                        .value
                        .len();
                    self.cursor_pos = cell_len;
                    self.enter_text();
                }
                _ => {}
            }
        }
        self.draw(out);
    }

    pub fn write_text<W: Write>(&mut self, key: crossterm::event::KeyCode, out: &mut W) {
        match key {
            KeyCode::Char(c) => {
                // println!("{}", key);

                let lines = self.cells[self.active_cell.row][self.active_cell.col]
                    .value
                    .split("\n")
                    .collect::<Vec<&str>>();

                if lines.last().unwrap().len() > self.cell_width - 2 {
                    self.cells[self.active_cell.row][self.active_cell.col]
                        .value
                        .push_str("\n")
                }
                println!("{}", self.cursor_pos);
                self.cells[self.active_cell.row][self.active_cell.col]
                    .value
                    .insert(self.cursor_pos, c);

                self.cursor_pos += 1;

                let cell_value = self.cells[self.active_cell.row][self.active_cell.col]
                    .value
                    .clone();

                if cell_value.starts_with("=") {
                    // self.enter_formula(self.active_cell.row, self.active_cell.col);
                    self.enter_formula(self.active_cell.row, self.active_cell.col);
                }
            }

            KeyCode::Backspace => {
                self.cursor_pos -= 1;
                self.cells[self.active_cell.row][self.active_cell.col]
                    .value
                    .remove(self.cursor_pos);
            }

            KeyCode::Left => {
                if self.cursor_pos != 0 {
                    self.cursor_pos -= 1;
                }
            }

            KeyCode::Right => {
                let cell_len = self.cells[self.active_cell.row][self.active_cell.col]
                    .value
                    .len();
                if self.cursor_pos != cell_len {
                    self.cursor_pos += 1;
                }
            }

            KeyCode::Tab => {
                self.cursor_pos = 0;
                self.text_edit = false;
            }
            _ => (),
        }

        self.draw(out);
    }
    pub fn get_active_cell(&self) -> Cell {
        self.cells[self.active_cell.row][self.active_cell.col].clone()
    }
    pub fn draw_axis<W: Write>(&self, out: &mut W) {
        for row in 0..self.cells.len() {
            out.execute(MoveTo(
                0,
                row as u16 * self.cell_height as u16 + AXIS_HEIGHT,
            ))
            .unwrap();
            print!("+");

            for _ in 0..=AXIS_WIDTH {
                print!("-");
            }

            out.execute(MoveTo(
                AXIS_WIDTH / 2,
                self.cell_height as u16 * row as u16
                    + self.cell_height as u16 / 2 as u16
                    + AXIS_HEIGHT,
            ))
            .unwrap();
            print!("{}", row + 1);
            for line in 1..self.cell_height {
                out.execute(MoveTo(
                    0,
                    row as u16 * self.cell_height as u16 + line as u16 + AXIS_HEIGHT,
                ))
                .unwrap();
                print!("|");
            }
        }

        print!("\n+-----");

        for col in 0..self.cells[0].len() {
            out.execute(MoveTo(col as u16 * self.cell_width as u16 + AXIS_WIDTH, 0))
                .unwrap();
            print!("+");

            for _ in 1..self.cell_width {
                print!("-");
            }

            out.execute(MoveTo(
                col as u16 * self.cell_width as u16 + (self.cell_width / 2) as u16 + AXIS_WIDTH,
                AXIS_HEIGHT / 2,
            ))
            .unwrap();
            print!("{}", (col as u8 + b'A') as char);
            for line in 1..AXIS_HEIGHT {
                out.execute(MoveTo(
                    col as u16 * self.cell_width as u16 + AXIS_WIDTH,
                    line as u16,
                ))
                .unwrap();
                print!("|");
            }
        }
    }

    pub fn draw_options<W: Write>(&self, out: &mut W) {
        out.execute(MoveTo(
            self.cells[0].len() as u16 * self.cell_width as u16 + 10,
            0,
        ))
        .unwrap();
        print!("Options");
    }

    pub fn draw<W: Write>(&mut self, out: &mut W) {
        // out.execute(terminal::Clear(ClearType::All)).unwrap();
        let rows = self.cells.len();
        let cols = if rows > 0 { self.cells[0].len() } else { 0 };

        for row in 0..=rows {
            for col in 0..=cols {
                out.execute(MoveTo(
                    (col * self.cell_width) as u16 + AXIS_WIDTH,
                    (row * self.cell_height) as u16 + AXIS_HEIGHT,
                ))
                .unwrap();
                print!("+");

                if col < cols {
                    for _ in 1..self.cell_width {
                        print!("-");
                    }
                }

                if row < rows {
                    for line_row in 1..self.cell_height {
                        out.execute(MoveTo(
                            (col * self.cell_width) as u16 + AXIS_WIDTH,
                            (row * self.cell_height + line_row) as u16 + AXIS_HEIGHT,
                        ))
                        .unwrap();
                        print!("|");
                    }
                }

                //              if self.active_cell.row == row && self.active_cell.col == col {
                //
                //}
            }
        }
        self.mark_selection(out);
        self.draw_axis(out);
        for row in 0..rows {
            for col in 0..cols {
                let content = self.cells[row][col].value.clone();

                out.execute(MoveTo(
                    (col * self.cell_width + 1) as u16 + AXIS_WIDTH,
                    (row * self.cell_height + 1) as u16 + AXIS_HEIGHT,
                ))
                .unwrap();
                if self.active_cell.row == row && self.active_cell.col == col {
                    let lines = content.lines().collect::<Vec<&str>>();

                    for (i, line) in lines.iter().enumerate() {
                        out.execute(MoveTo(
                            col as u16 * self.cell_width as u16 + 1 as u16 + AXIS_WIDTH,
                            row as u16 * self.cell_height as u16 + i as u16 + 1 + AXIS_HEIGHT,
                        ))
                        .unwrap();
                        print!("\x1b[7m{}\x1b[0m", line);
                        //print!("{}", line.with(Color::Blue))
                    }
                } else {
                    let lines = content.lines().collect::<Vec<&str>>();
                    let cell = self.cells[row][col].clone();
                    for (i, line) in lines.iter().enumerate() {
                        out.execute(MoveTo(
                            col as u16 * self.cell_width as u16 + 1 as u16 + AXIS_WIDTH,
                            row as u16 * self.cell_height as u16 + i as u16 + 1 + AXIS_HEIGHT,
                        ))
                        .unwrap();
                        let color = cell.color;
                        if cell.formula {
                            print!(
                                "{}",
                                self.enter_formula(row, col).unwrap_or("Error".to_string())
                            );
                        } else {
                            print!("{}", line.with(color))
                        }
                    }
                }
            }
        }
        // out.execute(terminal::Clear(ClearType::UntilNewLine))
        //     .unwrap();
        // for _ in 0..=self.cell_height - 1 {
        //     out.execute(MoveTo(
        //         (self.cells[0].len() + 10) as u16,
        //         (self.cells.len() + 10) as u16,
        //     ))
        //     .unwrap();
        //     print!("{}, {}", self.active_cell.row, self.active_cell.col);
        // }

        //self.draw_options(out);
    }

    pub fn mark_selection<W: Write>(&self, out: &mut W) {
        for line in 1..self.cell_height {
            out.execute(MoveTo(
                ((self.active_cell.col * self.cell_width) + 1) as u16 + AXIS_WIDTH,
                ((self.active_cell.row * self.cell_height) + line) as u16 + AXIS_HEIGHT,
            ))
            .unwrap();
            println!("{}", "x".repeat(self.cell_width - 1).on_grey().grey());
        }
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: &str) {
        self.cells[row][col].value = value.to_string();
    }

    pub fn enter_text(&mut self) {
        self.text_edit = true;
    }
}
