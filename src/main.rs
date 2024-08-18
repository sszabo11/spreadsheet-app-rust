use crossterm::{
    cursor::{self},
    event::{self, Event, KeyCode, KeyEventKind},
    terminal, ExecutableCommand,
};
use std::io::{stdout, Write};
mod cell;
mod spreadsheet;
use spreadsheet::Spreadsheet;

fn main() -> Result<(), String> {
    let cell_width = 12;
    let cell_height = 3;

    let mut spreadsheet = Spreadsheet::new(10, 6, cell_width, cell_height);
    let mut stdout = stdout();
    let terminal_size = terminal::size().unwrap();
    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::Hide).unwrap();

    println!("{:?}", terminal_size);
    spreadsheet.set_value(0, 0, "line\n2nd line");
    spreadsheet.set_value(0, 1, "B1");
    spreadsheet.set_value(1, 0, "A2");
    spreadsheet.set_value(1, 1, "B2");

    spreadsheet.draw(&mut stdout);
    stdout.flush().unwrap();

    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                spreadsheet.handle_key_press(key.code, &mut stdout);
                if spreadsheet.text_edit {
                    spreadsheet.write_text(key.code, &mut stdout)
                }
                match key.code {
                    KeyCode::Esc => break,
                    _ => (),
                }
            }
        }
    }

    stdout.execute(cursor::Show).unwrap();

    Ok(())
}
