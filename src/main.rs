mod config;
mod inputs;
mod snake;
mod table;
mod ui;
mod vec;

use config::SPEED;
use inputs::get_input;
use std::error::Error;
use table::Table;
use ui::*;

use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();
    let mut player_move: char = 'a';

    clear_screen();

    // Game loop
    while !table.lose {
        player_move = match get_input()? {
            ' ' => player_move,
            input => input,
        };
        sleep(Duration::from_millis(SPEED));
        table.snake.move_snake(player_move);
        table.update();
        reset_cursor();
        display_txt(format!("Apples: {} \n", table.apple_counter));
        table.render();
    }
    Ok(())
}
