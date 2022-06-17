use crossterm::{event::{read, Event, KeyCode}};

// Get WASD input from keyboard
pub fn get_input() -> crossterm::Result<char> {
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