mod config;
mod inputs;
mod snake;
mod table;
mod ui;
mod vec;

use config::SPEED;
use inputs::get_input;
use std::error::Error;
use std::process;
use table::Table;
use ui::*;

use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let mut table = Table::new();
    let mut player_input: char = 'a';

    clear_screen();

    // Game loop
    while !table.lose {
        player_input = match get_input()? {
            ' ' => player_input,
            'q' => {
                // Exit from the program
                display_txt("\nBye!\n".to_string());
                process::exit(0);
            }
            input => input,
        };
        sleep(Duration::from_millis(SPEED));
        table.snake.move_snake(player_input);
        table.update();
        reset_cursor();
        display_txt(format!("Apples: {} \n", table.apple_counter));
        table.render();
    }
    Ok(())
}
