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
    let sheet = app.spreadsheet.id.clone();
    let cells = app
        .database
        .get_cells(&format!("spreadsheet:{}", sheet))
        .unwrap();

    app.spreadsheet.load_cells(cells);

    //app.spreadsheet.draw(&mut stdout);
    app.home.draw(&mut stdout);
    stdout.flush().unwrap();
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
                    AppMode::Home => {
                        app.home.focus(&mut stdout, key.code);
                    }
                }
                match key.code {
                    KeyCode::Char(c) => match c {
                        ':' => {
                            app.spreadsheet.select_color = Color::DarkGrey;
                            app.spreadsheet.draw(&mut stdout);
                            app.mode = AppMode::Command;
                            app.command.input.push_str(":");
                            app.command.draw(&mut stdout);
                        }
                        '+' => {
                            if app.mode == AppMode::Home {
                                let name = app.home.create_sheet(&mut stdout);
                                print!("{}", name);
                                app.database.create_sheet(&name);
                                let sheets = app.database.get_sheets().unwrap();
                                app.home.sheets = sheets;

                                app.load_sheet(&mut stdout, &name);
                            }
                        }
                        _ => {}
                    },
                    KeyCode::Enter => match app.mode {
                        AppMode::Home => {
                            app.open_sheet(&mut stdout);
                            let sheet = app.spreadsheet.id.clone();
                            let cells = app
                                .database
                                .get_cells(&format!("spreadsheet:{}", sheet))
                                .unwrap();
                            app.spreadsheet.load_cells(cells);
                            app.spreadsheet.draw(&mut stdout);
                        }
                        _ => {
                            if app.command.input.starts_with(":") {
                                app.handle_command()
                            } else if app.command.input.starts_with("/") {
                                app.handle_search()
                            }
                        }
                    },
                    KeyCode::Esc => {
                        if app.mode == AppMode::Home {
                            break;
                        } else {
                            //app.home.focus(&mut stdout, key.code);
                            app.clear_screen(&mut stdout);
                            app.mode = AppMode::Home;
                            app.home.draw(&mut stdout);
                        }
                    }
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
