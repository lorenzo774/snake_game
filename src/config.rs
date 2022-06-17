use crossterm::style::Color;

pub const COLS: usize = 30;
pub const ROWS: usize = 15;
pub const BG_CHAR: char = '-';
pub const BG_COL: Color = Color::Blue;
pub const APPLE: char = '@';
pub const APPLE_COL: Color = Color::Red;
pub const SNAKE: char = '#';
pub const SNAKE_COL: Color = Color::Green;
pub const SNAKE_SIZE: u8 = 3;
pub const SPEED: u64 = 200;