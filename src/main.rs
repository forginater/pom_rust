mod user_input;
use user_input::{
    get_break_interval_len, get_num_intervals, get_planned_action, get_work_interval_len,
};
mod timer;
mod timer_no_pause;
mod util;

use timer::run_pomodoro;
use util::{clear_screen, setup_panic_handler};

use std::env;
use std::time::Duration;

fn main() {
    // Get command line args
    let args: Vec<String> = env::args().collect();

    // Check if the first argument is 'h' for help
    if args.len() == 2 && args[1] == "h" {
        println!("Usage: rust_pom <num_intervals> <interval_len> <break_interval> <activity>");
        println!("Run with no arguments for interactive mode.");
        return;
    }

    // Setup panic handler to disable raw_mode if program exits in panic state
    setup_panic_handler();
    // Clear the screen and move the cursor to the top
    clear_screen();

    match args.len() {
        1 => {
            // No additional arguments => Run interactive input
            let num_intervals = get_num_intervals();
            let interval_len = get_work_interval_len();
            let break_interval = get_break_interval_len();
            let _activity = get_planned_action();
            run_pomodoro(interval_len, num_intervals, break_interval);
        }
        5 => {
            // Parse command line arguments for Pomodoro settings
            let num_intervals = args[1]
                .parse::<usize>()
                .expect("Invalid number for intervals");
            let interval_len =
                Duration::from_secs(args[2].parse::<u64>().expect("Invalid interval length") * 60);
            let break_interval = Duration::from_secs(
                args[3]
                    .parse::<u64>()
                    .expect("Invalid break interval length")
                    * 60,
            );
            let _activity = &args[4];
            run_pomodoro(interval_len, num_intervals, break_interval);
        }
        _ => {
            eprintln!("Invalid arguments: For help, run: `rust_pom h`");
            return;
        }
    }
}
