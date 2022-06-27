mod reader;
mod editor;
use crossterm::terminal;
use editor::Editor;

struct CleanUp;

// Cleans up at end of program or panic.
// It is called when an instance of CleanUp goes
// out of scope.
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to turn off raw mode :(");
    }
}


fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("Failed to turn on raw mode.");
    let editor = Editor::new();

    while editor.run()? {}
    Ok(())
}
