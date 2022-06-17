use crossterm::{event::{poll, read, Event, KeyCode}};
use std::time::Duration;
use crate::config::SPEED;

// Get WASD input from keyboard
pub fn get_input() -> crossterm::Result<char> {
    const WASD: [char; 4] = ['w', 'a', 's', 'd'];
    if poll(Duration::from_millis(SPEED))? {
        let event = read()?;
        // Check WASD input
        for input in WASD {
            if event == Event::Key(KeyCode::Char(input).into()) {
                return Ok(input);
            }
        }
        return Ok(' ')
    }
    Ok(' ')
} 