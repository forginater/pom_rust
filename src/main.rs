mod user_input;
use user_input::{
    get_break_interval_len, get_num_intervals, get_planned_action, get_work_interval_len,
};
mod timer;
mod timer_no_pause;
mod util;

use timer::run_pomodoro;
use util::{clear_screen, setup_panic_handler};

fn main() {
    // Setup panic handler to disable raw_mode if program exits in panic state
    setup_panic_handler();
    // Clear the screen and move the cursor to the top
    clear_screen();

    // Get user input
    let num_intervals = get_num_intervals();
    let interval_len = get_work_interval_len();
    let break_interval = get_break_interval_len();
    let _activity = get_planned_action();

    // Run the pomodoro timer
    run_pomodoro(interval_len, num_intervals, break_interval);
}
