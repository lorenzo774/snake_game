use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    Result,
    event::{read, Event, KeyCode}
    // terminal::{Clear, ClearType}
};
use std::io::{stdout};

fn print_table(table: [[char; 20]; 10]) {
    let mut res = String::new();
    for row in table {
        for cell in row {
            res.push(cell);
        }
        res.push('\n');
    }
    execute!(
        stdout(),
        // Clear(ClearType::All),
        SetForegroundColor(Color::Blue),
        Print(res),
        ResetColor
    ).unwrap();
}

// Get WASD input from keyboard
fn get_input() -> crossterm::Result<char> {
    const WASD: [char; 4] = ['w', 'a', 's', 'd'];
    loop {
        let event = read()?;
        // Check WASD input
        for input in WASD {
            if event == Event::Key(KeyCode::Char(input).into()) {
                return Ok(input);
            }
        }
    }
}

// TODO: Move snake
fn move_snake(player_move: char) {
    println!("{}", player_move);
}

fn main() -> Result<()> {
    // Constants
    const COLS: usize = 20;
    const ROWS: usize = 10;
    const BG_CHAR: char = '-';
    const APPLE: char = '@';
    // const SNAKE: char = '#';    
    // const SNAKE_START_SIZE: u8 = 3;

    // TODO: Generate in random position the snake
    let gen_table = || [[BG_CHAR; COLS] ; ROWS];
    let mut table = gen_table();

    // Set apple first position
    table[4][4] = APPLE;
    
    let winning = false;

    let mut player_move: char;

    // TODO: Handle the loop (Handle inputs and apple taken by the snake)
    while !winning {
        print_table(table);
        player_move = get_input()?;
        move_snake(player_move);
    }
    Ok(())
}
