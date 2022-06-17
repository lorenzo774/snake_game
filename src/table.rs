use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor},
};
use std::io::{stdout};
use rand::Rng;

use crate::config::*;
use crate::snake::Snake;
use crate::vec::Vec2;

fn gen_random_pos() -> Vec2<u8> {
    let mut rng = rand::thread_rng();
    Vec2 {
        x: rng.gen_range(0..COLS) as u8, 
        y: rng.gen_range(0..ROWS) as u8 
    }
} 

pub struct Table {
    pub cells: [[char; COLS] ; ROWS],
    pub snake: Snake,
    pub apple: Vec2<u8>,
    pub lose: bool,
    pub apple_counter: i32
}
impl Table {
    pub fn new() -> Table {
        Table {
            cells: [[BG_CHAR; COLS]; ROWS],
            snake: Snake::new(),
            apple: gen_random_pos(),
            lose: false,
            apple_counter: 0
        }
    }

    fn clear(&mut self) {
        for i in 0..COLS {
            for j in 0..ROWS {
                self.cells[j][i] = BG_CHAR
            }
        }
    }

    fn add_entities(&mut self) {
        for coord in &self.snake.cells_pos {
            self.cells[coord.y as usize][coord.x as usize] = SNAKE;
        }
        self.cells[self.apple.y as usize][self.apple.x as usize] = APPLE;
    }

    fn spawn_rnd_apple(&mut self) {
        self.apple = gen_random_pos()
    }

    fn snake_colliding(&mut self) {
        let snake_head = self.snake.get_snake_head(); 
        match self.cells[snake_head.y as usize][snake_head.x as usize] {
            APPLE => {
                self.spawn_rnd_apple();
                self.apple_counter += 1;
                self.snake.add_cell();
            },
            SNAKE => {
                self.lose = true;
            }
            _ => (),
        }
    }

    pub fn render(&mut self) {
        self.clear();
        self.add_entities();
        for row in self.cells {
            for cell in row {
                let color = match cell {
                    APPLE => APPLE_COL,
                    SNAKE => SNAKE_COL,
                    _ => BG_COL
                };
                execute!(
                    stdout(),
                    SetForegroundColor(color),
                    Print(cell),
                    ResetColor
                ).unwrap();
            }
            println!();
        }
    }

    pub fn update(&mut self) {
        self.snake_colliding();
        // self.render();
    }

}