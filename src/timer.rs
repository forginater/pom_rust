use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::Local;
use std::io::{self, Write};
use std::time::Duration; // Trait needs to be in scope use stdout.flush().... ??

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};

enum IntervalType {
    Work,
    Break,
}

// Run the timer alternating between num_intervals of interval_len seconds and break intervals for break_len seconds.
pub fn run_pomodoro(
    work_interval_len: Duration,
    num_intervals: usize,
    break_interval_len: Duration,
) {
    // Enable raw mode to capture user input without requiring user to press Enter
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    timer_logic(work_interval_len, num_intervals, break_interval_len);
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn get_now() -> DelayedFormat<StrftimeItems<'static>> {
    return Local::now().format("%H:%M");
}

fn timer_logic(work_interval_len: Duration, num_intervals: usize, break_interval_len: Duration) {
    println!("\n@{}: Start Pomodoro", get_now());

    // Loop through each interval
    for interval in 1..=num_intervals {
        // Run work interval
        interval_countdown(IntervalType::Work, work_interval_len, interval);

        // Run Work interval if requested except after the last work interval
        if break_interval_len > Duration::from_secs(0) && interval < num_intervals {
            interval_countdown(IntervalType::Break, break_interval_len, interval);
        }
    }
    let total_work_duration = num_intervals as u64 * work_interval_len.as_secs();
    println!(
        "\n\r@{}: Pomodoro completed: Total time working = {}",
        get_now(),
        total_work_duration
    );
}

fn interval_countdown(interval_type: IntervalType, duration: Duration, interval_number: usize) {
    let mut remaining: u64 = duration.as_secs();
    let mut is_paused: bool = false;
    while remaining > 0 {
        // Non-blocking check for user input to pause the timer
        //  - If an event is detected within 50ms, poll returns true
        //  - NOTE: this runs approximately once per loop iteration, however keyboard input during the "sleep" duration will is queued in system's input buffer
        if event::poll(Duration::from_millis(1)).unwrap() {
            // event::read returns next available user input
            let input_event = crossterm::event::read().unwrap();
            // Use a match statement to check for Key events (keyboard)
            match input_event {
                Event::Key(KeyEvent {
                    code, modifiers, ..
                }) => {
                    // Check if the `code` KeyEvent is 'p'
                    match code {
                        KeyCode::Char('p') => {
                            // toggle is_paused
                            is_paused = !is_paused;
                            // If pausing notify user, resuming clear preceding two lines
                            if is_paused {
                                print!("\n\rPaused. Press 'p' again to resume");
                            } else {
                                print!("\r \x1B[K"); // Clear current line
                                print!("\x1B[A\x1B[K"); // Clear preceding line
                            }
                            io::stdout().flush().unwrap();
                            // exit this iteration of the loop if pausing or resuming
                            continue;
                        }
                        KeyCode::Char('c') if modifiers.contains(event::KeyModifiers::CONTROL) => {
                            // Handle Ctrl+C to exit
                            println!("Exiting...");
                            terminal::disable_raw_mode().expect("Failed to disable raw mode");
                            std::process::exit(0);
                        }
                        _ => {}
                    };
                }
                _ => {}
            }
        }

        // If timer running: update UI, sleep and proceed with countdown
        if !is_paused {
            display_countdown(&interval_type, interval_number, remaining);
            std::thread::sleep(Duration::from_millis(1000));
            remaining -= 1;
        } else {
            // Avoid straining CPU with busy loop while paused and make resume polling more frequent and responsive
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    if !is_paused {
        print_interval_done_message(&interval_type, interval_number);
    }
}

/*
    Functions to print the countdown state:
    relies on carriage return "\r" for the display countdown and ASCI code "\x1B[K" to clear the line
*/

fn display_countdown(interval_type: &IntervalType, interval_number: usize, remaining: u64) {
    let interval_label = match interval_type {
        IntervalType::Work => "Work",
        IntervalType::Break => "Break",
    };
    print!(
        "\r{} Interval #{}: {} seconds remaining",
        interval_label, interval_number, remaining
    );
    // Neccessary because Rust's stdout is line-buffered by default
    std::io::stdout().flush().expect("Failed to flush stdout");
}

fn print_interval_done_message(interval_type: &IntervalType, interval_number: usize) {
    let done_msg = match interval_type {
        IntervalType::Work => {
            format!(
                "\r@{}: Completed Interval #{}  \x1B[K",
                get_now(),
                interval_number
            )
        }
        IntervalType::Break => format!("\r@{}: Break Done \x1B[K", get_now()),
    };
    println!("{}", done_msg);
}
