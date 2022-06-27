use crate::reader::Reader;
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::{cursor, event, execute, terminal};
use crossterm::terminal::ClearType;
use std::io::stdout;


pub fn clear_screen() -> crossterm::Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::All))?;
    execute!(stdout(), cursor::MoveTo(0, 0))
}


pub struct Editor {
    reader: Reader,
    window_size: (usize, usize),
}


impl Editor {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();

        Self { 
            reader: Reader, 
            window_size: win_size,
        }
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

    fn draw_rows(&self) {
        for _ in 0..self.window_size.0 {
            println!("~\r");
        }
    } 

    pub fn run(&self) -> crossterm::Result<bool> {
        clear_screen().unwrap();
        self.draw_rows();
        self.process_keystroke()
    }
}
