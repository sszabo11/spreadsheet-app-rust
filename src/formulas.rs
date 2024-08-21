use std::ascii::AsciiExt;

use crate::{
    cell::Cell,
    spreadsheet::{self, Spreadsheet},
};

#[derive(Clone)]
pub struct Formula {
    pub operation: FormulaType,
    pub range: Vec<(usize, usize)>,
}

impl Formula {
    pub fn evaluate(&self, spreadsheet: &Spreadsheet) -> String {
        let values: Vec<f64> = self
            .range
            .iter()
            .map(|&(row, col)| {
                spreadsheet.cells[row][col]
                    .value
                    .parse::<f64>()
                    .unwrap_or(0.0)
            })
            .collect();

        match self.operation {
            FormulaType::SUM => {
                let value: f64 = values.iter().sum();
                return value.to_string();
            }

            FormulaType::PRODUCT => {
                let value: f64 = values.iter().product();
                return value.to_string();
            }
            FormulaType::QUOTIENT => "".to_string(),
            FormulaType::DIFFERENCE => "".to_string(),
        }
    }
}

pub trait FormulaHandler {
    fn enter_formula(&mut self, row: usize, col: usize) -> Option<String>;

    fn parse_range(&self, value: &str) -> Option<String>;

    fn convert_range(&self, range: &str) -> Option<Vec<(usize, usize)>>;

    fn parse_operation(&self, value: &str) -> Option<FormulaType>;

    fn evaluate(&self, operation: FormulaType, range: Vec<(usize, usize)>) -> Result<String, ()>;

    fn get_cords_from_ref(&self, range_str: &str) -> Option<(usize, usize)>;
}

impl FormulaHandler for Spreadsheet {
    fn enter_formula(&mut self, row: usize, col: usize) -> Option<String> {
        let value = self.cells[row][col].value.clone();

        let operation: Option<FormulaType> = self.parse_operation(&value);
        if let None = operation {
            return None;
        };

        let operation = operation.unwrap();
        let range_str: Option<String> = self.parse_range(&value);

        if let None = range_str {
            return None;
        };
        let range: Option<Vec<(usize, usize)>> = self.convert_range(&range_str.unwrap());

        if let Some(range) = range {
            self.cells[row][col].formula = Some(Formula {
                operation: operation.clone(),
                range: range.clone(),
            });
            let result = self.evaluate(operation, range).unwrap();
            //println!("Result: {}", result);
            return Some(result);
        } else {
            None
        }
    }

    fn parse_range(&self, value: &str) -> Option<String> {
        let mut range = value.split("(").collect::<Vec<&str>>()[1].to_string();
        if !range.ends_with(")") {
            return None;
        };

        range.pop();

        if !range.contains(":") {
            return None;
        }

        return Some(range);
        // let range: Vec<&str> = range.split(":").collect::<Vec<&str>>();

        // let range = self.convert_range(&range);
    }

    fn convert_range(&self, range: &str) -> Option<Vec<(usize, usize)>> {
        let (start, end) = range.split_once(":").unwrap();

        let start = self.get_cords_from_ref(start);
        let end = self.get_cords_from_ref(end);
        let ok = match (start, end) {
            (Some(_), Some(_)) => true,
            _ => false,
        };
        if !ok {
            return None;
        };

        let start = start.unwrap();
        let end = end.unwrap();

        let mut range = Vec::new();
        if start.0 == end.0 {
            // On same row
            for col in start.1..=end.1 {
                range.push((start.0, col));
            }
        } else if start.1 == end.1 {
            // On same column
            for row in start.0..=end.0 {
                range.push((row, start.1));
            }
        } else {
            return None;
        }

        Some(range)
    }

    fn get_cords_from_ref(&self, range_str: &str) -> Option<(usize, usize)> {
        let mut chars = range_str.chars();
        let col = chars.nth(0).unwrap();
        let row = chars.nth(0).unwrap();

        if !col.is_alphabetic() {
            return None;
        };

        let col_num = col as usize - 'A' as usize;
        let row_num = row.to_digit(10).unwrap() as usize - 1;
        Some((row_num, col_num))
    }

    fn evaluate(&self, operation: FormulaType, range: Vec<(usize, usize)>) -> Result<String, ()> {
        match operation {
            FormulaType::SUM => {
                let mut result: u16 = 0;
                for (row, col) in range {
                    let value = self.cells[row][col].value.clone();
                    let value = value.parse::<u16>().unwrap_or(0);
                    result += value;
                }

                return Ok(result.to_string());
            }
            FormulaType::PRODUCT => {
                let mut result: u16 = 1;

                for (row, col) in range {
                    let value = self.cells[row][col].value.clone();
                    let value = value.parse::<u16>().unwrap_or(0);
                    result *= value;
                }

                return Ok(result.to_string());
            }
            _ => Err(()),
        }
    }

    fn parse_operation(&self, value: &str) -> Option<FormulaType> {
        if !value.starts_with("=") {
            return None;
        };
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        let value = chars.as_str();

        let operation_str = value.split("(").collect::<Vec<&str>>()[0];

        match operation_str {
            "SUM" => return Some(FormulaType::SUM),
            "PRODUCT" => {
                return Some(FormulaType::PRODUCT);
            }
            _ => return None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum FormulaType {
    SUM,
    PRODUCT,
    DIFFERENCE,
    QUOTIENT,
}

pub fn letter_pos(letter: char) -> Option<u32> {
    if letter.is_alphabetic() {
        Some(letter.to_ascii_uppercase() as u32 - 64)
    } else {
        None
    }
}
