mod editor;
use editor::Editor;
use crossterm::terminal;
struct CleanUp;

// Cleans up at end of program or panic.
// It is called when an instance of CleanUp goes
// out of scope.
impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Failed to turn off raw mode :(");
        editor::clear_screen();
    }
}


fn main() -> crossterm::Result<()> {
    let _cleanup = CleanUp;
    let mut editor = Editor::new();
    
    terminal::enable_raw_mode().expect("Failed to enable raw mode.");
    editor.run();

    Ok(())
}
