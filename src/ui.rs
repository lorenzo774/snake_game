use crossterm::{
    execute,
    style::{Print, ResetColor, Color, SetForegroundColor},
    terminal::*
};
use std::io::{stdout};

pub fn clear_screen() {
    execute!(stdout(),Clear(ClearType::All)).unwrap();
}

pub fn display_txt(txt: String) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Cyan),
        Print(txt + "\n"),
        ResetColor
    ).unwrap();
}