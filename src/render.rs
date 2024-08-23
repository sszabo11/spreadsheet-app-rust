use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal, ExecutableCommand,
};

use crate::app::{AppMode, AppState};

pub fn render_app(app: &mut AppState) -> Result<(), String> {
    let mut stdout = stdout();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::Hide).unwrap();

    app.spreadsheet.set_value(0, 0, "line\n2nd line");
    app.spreadsheet.set_value(0, 1, "B1");
    app.spreadsheet.set_value(0, 3, "4");
    app.spreadsheet.set_value(1, 3, "7");
    app.spreadsheet.set_value(1, 0, "A2");
    app.spreadsheet.set_value(1, 1, "B2");

    app.spreadsheet.draw(&mut stdout);
    stdout.flush().unwrap();

    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match app.mode {
                    AppMode::Normal => {
                        app.spreadsheet.focus(&mut stdout, key.code);
                    }
                    AppMode::Command => {
                        //app.command.focus();
                    }
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
