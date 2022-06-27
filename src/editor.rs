use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, execute, terminal, cursor};
use crossterm::terminal::ClearType;
use std::io::{Write, stdout};
use core::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct Editor {
    x: u16,
    y: u16,
    max_x: u16,
    max_y: u16
}


pub fn clear_screen() {
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
    move_cursor(0, 0);
}

fn move_cursor(x: u16, y: u16) {
    execute!(stdout(), cursor::MoveTo(x, y)).unwrap();
}


impl Editor {
    pub fn new() -> Self {
        clear_screen();

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

    // Updates cursor to cursor x and cursor y.
    fn update_cursor(&self) {
        move_cursor(self.x, self.y);
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

    // Makes a newline.
    fn newline(&mut self) {
        self.y += 1;
        self.x = 0;
        self.update_cursor();
    }

    fn show_start_screen(&self) {
        let version: String = String::from(VERSION);
        move_cursor((self.max_x - version.len() as u16) / 3, 2);
        println!("{}", format!("NexEdit - {}", VERSION));

        loop {
            if event::poll(Duration::from_millis(500)).unwrap()  {
                clear_screen();
                move_cursor(0, 0);
                break;
            }
        }

        self.draw_rows();
        move_cursor(0, 0);          // Bring cursor back up.
    }

    // Draws '~'
    fn draw_rows(&self) {
        for i in 1..self.max_y {
            println!("~\r");
        }
    }

    pub fn run(&mut self) {
        self.show_start_screen();

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
                        print!("\r\n ");
                        self.newline();
                    }

                    KeyEvent {
                        code: KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE
                    } => self.dec_cursor_pos(),

                    _ => {
                        if let KeyCode::Char(c) = event.code {
                            print!("{}", c);
                            stdout().flush().unwrap();               // Flush to terminal.
                        }

                        self.inc_cursor_pos();
                    }
                }
            }
        }
    }
}
