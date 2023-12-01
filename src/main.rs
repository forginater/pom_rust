mod user_input;
use user_input::{
    get_break_interval_len, get_num_intervals, get_planned_action, get_work_interval_len,
};
mod timer;
mod timer_no_pause;
use timer::run_pomodoro;

use std::panic;

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{self, Clear, ClearType},
};

use std;

fn main() {
    let default_panic = panic::take_hook();
    panic::set_hook(Box::new(move |panic_reason| {
        // assign result to blubli
        let _ = terminal::disable_raw_mode();
        // Make sure to run default panic hook
        default_panic(panic_reason);
    }));

    // Clear the screen and move the cursor to the top
    execute!(std::io::stdout(), Clear(ClearType::All), MoveTo(0, 0))
        .expect("Failed to clear screen and move to top");

    // Pom takes user input (numIntervals, intervalLen) and runs a timer numIntervals times each for a length of intervalLen
    // Get user input
    let num_intervals = get_num_intervals();
    let interval_len = get_work_interval_len();
    let break_interval = get_break_interval_len();
    let _activity = get_planned_action();

    // Run the pomodoro timer
    run_pomodoro(interval_len, num_intervals, break_interval);
    // timer_logic(interval_len, num_intervals, break_interval);
}
