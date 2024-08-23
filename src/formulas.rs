use crate::spreadsheet::{self, Spreadsheet};

pub trait FormulaHandler {
    fn is_formula(&mut self, value: &str, row: usize, col: usize) -> Option<bool>;

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

        let operation = self.parse_operation(&value)?;

        let range_str = self.parse_range(&value)?;

        let range = self.convert_range(&range_str)?;

        self.cells[row][col].formula = true;
        let result = self.evaluate(operation, range).unwrap();
        Some(result)
    }

    fn is_formula(&mut self, value: &str, row: usize, col: usize) -> Option<bool> {
        let result = self.enter_formula(row, col);

        let operation = self.parse_operation(&value)?;

        let range_str = self.parse_range(&value)?;

        let range = self.convert_range(&range_str)?;

        Some(true)
    }

    fn parse_range(&self, value: &str) -> Option<String> {
        let range = value.split("(").nth(1)?.strip_suffix(')')?;

        if range.contains(":") {
            Some(range.to_string())
        } else {
            None
        }
    }

    fn convert_range(&self, range: &str) -> Option<Vec<(usize, usize)>> {
        let (start, end) = range.split_once(":")?;

        let start = self.get_cords_from_ref(start)?;
        let end = self.get_cords_from_ref(end)?;

        let range: Vec<_> = if start.0 == end.0 {
            // On same row

            (start.1..=end.1).map(|col| (start.0, col)).collect()
        } else if start.1 == end.1 {
            // On same column

            (start.0..=end.0).map(|row| (row, start.1)).collect()
        } else {
            return None;
        };

        Some(range)
    }

    fn get_cords_from_ref(&self, range_str: &str) -> Option<(usize, usize)> {
        let mut chars = range_str.chars();
        let col = chars.next()?.to_ascii_uppercase();
        let row = chars.next()?.to_digit(10)?;

        if !col.is_alphabetic() {
            return None;
        };

        let col_num = col as usize - 'A' as usize;
        let row_num = row as usize - 1;
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

        let operation_str = value
            .trim_matches(|c| c == '=' || c == ')')
            .split_once("(")?
            .0; // =SUM(A1:A5) -> ["SUM", "A1:A5"]

        match operation_str {
            "SUM" => Some(FormulaType::SUM),
            "PRODUCT" => Some(FormulaType::PRODUCT),
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
