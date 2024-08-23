use std::io::{stdout, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    style::Color,
    terminal, ExecutableCommand,
};

use crate::{
    app::{AppMode, AppState},
    command,
};

pub fn render_app(app: &mut AppState) -> Result<(), String> {
    let mut stdout = stdout();

    stdout
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.execute(cursor::Hide).unwrap();

    //app.spreadsheet.set_value(0, 0, "line\n2nd line");
    //app.spreadsheet.set_value(0, 1, "B1");
    //app.spreadsheet.set_value(0, 3, "4");
    //app.spreadsheet.set_value(1, 3, "7");
    //app.spreadsheet.set_value(1, 0, "A2");
    //app.spreadsheet.set_value(1, 1, "B2");
    let cells = app.database.get_cells("spreadsheet:1").unwrap();

    app.spreadsheet.load_cells(cells);

    app.spreadsheet.draw(&mut stdout);
    stdout.flush().unwrap();
    app.database.get_cells("spreadsheet:1").unwrap();
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match app.mode {
                    AppMode::Normal => {
                        app.spreadsheet.focus(&mut stdout, key.code);
                    }
                    AppMode::Command => {
                        app.command.focus(&mut stdout, key.code);
                    }
                }
                match key.code {
                    KeyCode::Char(c) => {
                        if c == ':' {
                            app.spreadsheet.select_color = Color::DarkGrey;
                            app.spreadsheet.draw(&mut stdout);
                            app.mode = AppMode::Command;
                            app.command.input.push_str(":");
                            app.command.draw(&mut stdout);
                        }
                    }
                    KeyCode::Enter => {
                        if app.command.input.starts_with(":") {
                            app.handle_command()
                        } else if app.command.input.starts_with("/") {
                            app.handle_search()
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {
                        if app.mode == AppMode::Command && app.command.input.len() == 0 {
                            app.spreadsheet.select_color = Color::Grey;
                            app.spreadsheet.draw(&mut stdout);
                            stdout.execute(cursor::Hide).unwrap();
                            app.mode = AppMode::Normal
                        }
                    }
                }
            }
        }
    }

    stdout.execute(cursor::Show).unwrap();
    Ok(())
}
