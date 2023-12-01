use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{self, Clear, ClearType},
};
use std;
use std::panic;

pub fn setup_panic_handler() {
    // Get the default panic handler
    let default_panic = panic::take_hook();
    // register custom panic hook
    panic::set_hook(Box::new(move |panic_reason| {
        // assign result to blubli
        let _ = terminal::disable_raw_mode();
        // Make sure to run default panic hook
        default_panic(panic_reason);
    }));
}

pub fn clear_screen() {
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
        .expect("Failed to clear screen and move to top");
}
