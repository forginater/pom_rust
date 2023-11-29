mod user_input;
use user_input::{
    get_break_interval_len, get_num_intervals, get_planned_action, get_work_interval_len,
};
mod timer;
use timer::timer_logic;
mod timer_pause;
use timer_pause::run_pomodoro;

fn main() {
    // Pom takes user input (numIntervals, intervalLen) and runs a timer numIntervals times each for a length of intervalLen
    // Get user input
    let num_intervals = get_num_intervals();
    let interval_len = get_work_interval_len();
    let break_interval = get_break_interval_len();
    let _activity = get_planned_action();

    // Run the pomodoro timer
    let mode = "pause";
    if mode == "pause" {
        run_pomodoro(interval_len, num_intervals, break_interval);
    }
    if mode == "resume" {
        timer_logic(interval_len, num_intervals, break_interval);
    }
}
