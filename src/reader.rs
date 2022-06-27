use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};

pub struct Reader;

impl Reader {
    pub fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if let Event::Key(event) = event::read()? {
                return Ok(event);
            }
        }
    }
}
