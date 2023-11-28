mod user_input;
use user_input::{
    get_break_interval_len, get_num_intervals, get_planned_action, get_work_interval_len,
};
mod timer;
use timer::timer_logic;
mod timer_pause;
use timer_pause::timer_logic_can_pause;
// TODO
// Add pause/resume functionality
// Add Prompt at end to check productivity, relevance etc
// write results to file or database
// prettify terminal output
// Utilise custom errors and implement fmt

fn main() {
    // TEST pause/resume functions
    // test_check_for_input();

    // Pom takes user input (numIntervals, intervalLen) and runs a timer numIntervals times each for a length of intervalLen
    // Get user input (numIntervals)
    let num_intervals = get_num_intervals();

    // Get user input (intervalLen)
    let interval_len = get_work_interval_len();

    // Get break interval
    let break_interval = get_break_interval_len();

    // Get planned activity
    let _activity = get_planned_action();

    // Run the pomodoro timer
    let mode = "pause";
    if mode == "pause" {
        timer_logic_can_pause(interval_len, num_intervals, break_interval);
    } else {
        timer_logic(interval_len, num_intervals, break_interval);
    }
}
