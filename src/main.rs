mod inputs;
mod entities;
mod config;

use inputs::get_input;
use std::error::Error;
use entities::{Table};

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Generate in random position the snake
    let mut table = Table::new();

    let winning = false;
    let mut player_move: char;

    // TODO: Handle the loop (Handle inputs and apple taken by the snake)
    while !winning {
        table.render();
        player_move = get_input()?;
        table.snake.move_snake(player_move);
    }
    Ok(())
}
