use crate::reader::Reader;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::event::*;
use crossterm::{cursor, event, execute, terminal};
use crossterm::terminal::ClearType;
use std::io::stdout;


fn clear_screen() -> crossterm::Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::All))?;
    execute!(stdout(), cursor::MoveTo(0, 0))
}


pub struct Editor {
    reader: Reader,
}


impl Editor {
    pub fn new() -> Self {
        Self { reader: Reader }
    }

    fn process_keystroke(&self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL
            } => return Ok(false),
            _ => {}
        }

        Ok(true)
    }

    pub fn run(&self) -> crossterm::Result<bool> {
        clear_screen();
        self.process_keystroke()
    }
}
