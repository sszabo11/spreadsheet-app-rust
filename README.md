![Spreadsheet](https://github.com/sszabo11/spreadsheets/blob/main/screenshots/spreadsheet.png?raw=true)

# 📝 Terminal Spreadsheet App 🦀

Welcome to the Terminal Spreadsheet App, a Rust-powered, low-level, terminal-based spreadsheet application! 🚀

## 🎉 Features

- **Keyboard Navigation**: Use arrow keys to move between cells effortlessly.
- **Custom Cell Colors**: Highlight cells with custom colors for better visibility.
- **Text Editing**: Enter and edit text directly in any cell.
- **Minimal Flicker**: Optimized rendering to minimize flicker during screen updates.
- **Simple formulas**: Currently Sum and Product formulas work to a range of cells.
- **Commands (In progress)**: Ability to enter command mode and **eventually** do stuff.

## 🏗️ Still working progress
- I still want to include all the future features listed below
- Still very buggy

## 🛠️ Installation

To get started, clone this repository and build the project using Cargo:

```bash
git clone https://github.com/sszabo11/spreadsheet-app-rust.git
cd spreadsheet-app-rust
cargo build --release
```

## 🧑‍💻 Usage
[](url)
Run the application with:

```bash
cargo run --release
```

## 🔮 Future Features
- **Save data to database:** Be able to save sheet to a database and come back to it
- **Complex formulas:** Support for advanced formulas.
- **Styling:** Apply different styles to cells (bold, italic, etc.).
- **Import Excel Sheets:** Import existing Excel sheets into the terminal app.
- **Export as Excel:** Export your spreadsheet as an Excel file.

## 🎮 Controls
- **Arrow Keys:** Navigate between cells.
- **Enter:** Start editing the selected cell.
- **Backspace:** Delete characters in the selected cell.
- **Esc:** Exit the application.

## 📦 Dependencies
This project uses the following crates:
- **crossterm**: For terminal input/output handling.

## 🥰 Feel free to use this
