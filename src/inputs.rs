use crate::config::SPEED;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

// Get WASD input from keyboard
pub fn get_input() -> crossterm::Result<char> {
    const KEYS: [char; 5] = ['q', 'w', 'a', 's', 'd'];
    if poll(Duration::from_millis(SPEED))? {
        let event = read()?;
        // Check KEYS input
        for input in KEYS {
            if event == Event::Key(KeyCode::Char(input).into()) {
                return Ok(input);
            }
        }
        return Ok(' ');
    }
    Ok(' ')
}
