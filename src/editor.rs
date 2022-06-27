use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, execute, terminal, cursor};
use crossterm::terminal::ClearType;
use std::io::{Write, stdout};


pub struct Editor {
    x: u16,
    y: u16,
    max_x: u16,
    max_y: u16
}


fn clear_screen() {
    execute!(stdout(), terminal::Clear(ClearType::All));
}


impl Editor {
    pub fn new() -> Self {
        clear_screen();
        execute!(stdout(), cursor::MoveTo(0, 0));

        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();

        Self {
            x: 0,
            y: 0,
            max_x: win_size.0 as u16,
            max_y: win_size.1 as u16,
        }
    }

    fn update_cursor(&self) -> crossterm::Result<()> {
        execute!(stdout(), cursor::MoveTo(self.x, self.y))
    }

    // Decrements cursor position.
    fn dec_cursor_pos(&mut self) {
        if self.x == 0 && self.y == 0 {
            return;
        } else if self.x == 0 {
            self.y -= 1;
        } else {
            self.x -= 1;
        }

        self.update_cursor();
    }

    fn inc_cursor_pos(&mut self) {
        if self.x >= self.max_x {
            self.x = 0;
            self.y += 1;
        } else {
            self.x += 1;
        }

        self.update_cursor();
    }

    fn newline(&mut self) {
        self.y += 1;
        self.x = 0;
        self.update_cursor();
    }

    pub fn run(&mut self) {
        loop {
            if let Event::Key(event) = event::read().expect("Failed to read key.") { 
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::CONTROL
                    } => break,

                    KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: event::KeyModifiers::NONE
                    } => {
                        print!("\r \n");
                        self.newline();
                    }

                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE
                    } => self.dec_cursor_pos(),

                    _ => {
                        self.inc_cursor_pos();
                        if let KeyCode::Char(c) = event.code {
                            print!("{}", c);
                            stdout().flush();               // Flush to terminal.
                        }
                    }
                }
            }
        }
    }
}
