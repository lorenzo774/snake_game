use crate::config::*;
use crate::vec::Vec2;
use rand::Rng;

#[derive(Debug)]
pub struct Snake {
    pub cells_pos: Vec<Vec2<u8>>  
}
impl Snake {
    pub fn get_snake_head(&mut self) -> Vec2<u8> {
        Vec2 {
            x: self.cells_pos[0].x,
            y: self.cells_pos[0].y,
        }
    }

    pub fn add_cell(&mut self) {
        self.cells_pos.push(Vec2 { x: self.cells_pos[self.cells_pos.len()-1].x+1, y: self.cells_pos[self.cells_pos.len()-1].y });
    }

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

    pub fn move_snake(&mut self, input: char) {
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
        // Create new body
        let mut new_body: Vec<Vec2<u8>> = Vec::new();
        for i in 0..self.cells_pos.len()-1 {
            new_body.push(
                Vec2 { 
                    y: self.cells_pos[i].y, 
                    x: self.cells_pos[i].x 
                });
            }
            for i in 0..self.cells_pos.len() {
                if i == 0 {
                    let mut head_mov = Vec2 {x: self.cells_pos[i].x, y: self.cells_pos[i].y};

                    if ((self.cells_pos[i].x as i8) + movement.x) as i8 >= COLS as i8 {
                        head_mov.x = 0;
                    }
                    else if ((self.cells_pos[i].y as i8) + movement.y) < 0 {
                        head_mov.y = (ROWS - 1) as u8;
                    }
                    else if ((self.cells_pos[i].y as i8) + movement.y) as i8 >= ROWS as i8 {
                        head_mov.y = 0;
                    }
                    else if (self.cells_pos[i].x as i8) + movement.x < 0 {
                        head_mov.x = (COLS - 1) as u8;
                    }
                    else {
                        head_mov.x = (head_mov.x as i8 + movement.x) as u8;
                        head_mov.y = (head_mov.y as i8 + movement.y) as u8;
                    }
                    
                    self.cells_pos[i].x = head_mov.x;
                    self.cells_pos[i].y = head_mov.y;
                }
                else {
                    self.cells_pos[i].x = new_body[i-1].x;
                    self.cells_pos[i].y = new_body[i-1].y;
                }
        }
    }
}