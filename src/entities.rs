use crossterm::{
    execute,
    style::{Print, ResetColor, SetForegroundColor},
    // terminal::{Clear, ClearType}
};
use std::io::{stdout};

use crate::config::*;
use rand::{Rng};

#[derive(Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}
impl PartialEq for Vec2<u8> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Vec2<u8> {}

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
    pub apple: Vec2<u8>
}
impl Table {
    pub fn new() -> Table {
        Table {
            cells: [[BG_CHAR; COLS]; ROWS],
            snake: Snake::new(),
            apple: gen_random_pos()
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
                    // Clear(ClearType::All),
                    SetForegroundColor(color),
                    Print(cell),
                    ResetColor
                ).unwrap();
            }
            println!();
        }
    }

}

#[derive(Debug)]
pub struct Snake {
    pub cells_pos: Vec<Vec2<u8>>  
}
impl Snake {
    pub fn new() -> Snake {
        let mut rng = rand::thread_rng();
        let mut random_cells: Vec<Vec2<u8>> = Vec::new();
        
        let max_x = COLS - SNAKE_SIZE as usize;
        let max_y = ROWS - SNAKE_SIZE as usize;
        
        // Create snake head
        random_cells.push(
            Vec2 {
                x: rng.gen_range(0..max_x) as u8, 
                y: rng.gen_range(0..max_y) as u8 
            }
        );
        
        // Create snake body
        for i in 1..SNAKE_SIZE {
            random_cells.push(
                Vec2 {
                    x: random_cells[0].x + i, 
                    y: random_cells[0].y
                }
            );
        }

        Snake { cells_pos: random_cells }
    }

    // TODO: Move snake
    pub fn move_snake(&mut self, input: char) {
        println!("Input={}", input);

        let movement: Vec2<i8> = match input {
            'w' => {
                Vec2 {
                    y: -1,
                    x: 0
                }
            },
            's' => {
                Vec2 {
                    y: 1,
                    x: 0
                }
            },
            'a' => {
                Vec2 {
                    y: 0,
                    x: -1
                }
            },
            'd' => {
                Vec2 {
                    y: 0,
                    x: 1
                }
            }
            _ => Vec2 { y: 0, x: 0 }
        };

        for i in 0..self.cells_pos.len() {
            if i == 0 {
                self.cells_pos[i].x = ((self.cells_pos[i].x as i8) + movement.x) as u8;
                self.cells_pos[i].y = ((self.cells_pos[i].y as i8) + movement.y) as u8;
            } 
            else {
                let cell_before = &self.cells_pos[i - 1];

                if cell_before == &self.cells_pos[i - 1] {
                    
                }
                else {
                }

                println!("Cell before={:?}", cell_before);
            }
        }

        println!("{:?}", self.cells_pos);
    }
}