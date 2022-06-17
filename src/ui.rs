use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::*,
};
use std::io::stdout;

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

pub fn reset_cursor() {
    execute!(stdout(), Hide, MoveTo(0, 0)).unwrap();
}

pub fn display_txt(txt: String) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print(txt + "\n"),
        ResetColor
    )
    .unwrap();
}
