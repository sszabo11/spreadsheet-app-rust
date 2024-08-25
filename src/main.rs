mod app;
mod cell;
use app::AppState;
mod formulas;
mod render;
mod spreadsheet;
use render::render_app;
mod command;
mod database;
mod home;
fn main() -> Result<(), String> {
    let cell_width = 12;
    let cell_height = 3;

    let mut app = AppState::new(cell_width, cell_height);

    render_app(&mut app).unwrap();

    Ok(())
}
