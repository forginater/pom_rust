use std::io::Write;
use std::time::Duration; // Trait needs to be in scope use stdout.flush().... ??

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal, ExecutableCommand,
};

enum IntervalType {
    Work,
    Break,
}

// Run the timer alternating between num_intervals of interval_len seconds and break intervals for break_len seconds.
pub fn timer_logic_can_pause(
    work_interval_len: Duration,
    num_intervals: usize,
    break_interval_len: Duration,
) {
    // Enable raw mode to capture user input without requiring user to press Enter
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
    timer_logic(work_interval_len, num_intervals, break_interval_len);
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn timer_logic(work_interval_len: Duration, num_intervals: usize, break_interval_len: Duration) {
    let interval_seconds = work_interval_len.as_secs();
    println!("\nPomodoro Timer Started: {num_intervals} intervals of {interval_seconds} \n");

    // Loop through each interval
    for interval in 1..=num_intervals {
        // Run work interval
        countdown(IntervalType::Work, work_interval_len, interval);

        // Run Work interval if requested except after the last work interval
        if break_interval_len > Duration::from_secs(0) && interval < num_intervals {
            countdown(IntervalType::Break, break_interval_len, interval);
        }
    }
    println!("\nPomodoro completed");
}

#[allow(unused)]
fn countdown(interval_type: IntervalType, duration: Duration, interval_number: usize) {
    let mut remaining: u64 = duration.as_secs();
    let mut is_paused: bool = false;

    while remaining > 0 {
        // Update the countdown in terminal each second
        if !is_paused {
            display_countdown(&interval_type, interval_number, remaining);
            std::io::stdout().flush().expect("Failed to flush stdout");
            remaining -= 1;
        }
        // Non-blocking check every 100ms for user input to pause the timer
        // If an event is available, poll returns true
        if event::poll(Duration::from_millis(50)).unwrap() {
            println!("Poll");
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
                            // toggle is_paused state and notify the user
                            is_paused = !is_paused;
                            if is_paused {
                                println!("\n\rPaused. Press 'p' again to resume.");
                            } else {
                                println!("\n\rResuming...");
                            }
                        }
                        KeyCode::Char('c') if modifiers.contains(event::KeyModifiers::CONTROL) => {
                            // Handle Ctrl+C to exit
                            println!("Exiting...");
                            std::process::exit(0);
                        }
                        _ => {}
                    };
                }
                _ => {}
            }
        }
        // wait until after polling to sleep to avoid delayed pause
        if !is_paused {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    print_interval_done_message(&interval_type, interval_number);
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
}

fn print_interval_done_message(interval_type: &IntervalType, interval_number: usize) {
    let done_msg = match interval_type {
        IntervalType::Work => {
            format!("\rInterval #{} done  \x1B[K", interval_number)
        }
        IntervalType::Break => format!("\rBreak Done \x1B[K"),
    };
    println!("{}", done_msg);
}

/*
    ####################################################################################
    Polling Approach: Fails due to std::io::stdin().read_line blocking program execution
    ####################################################################################
*/

#[allow(unused)]
fn countdown_fail(interval_type: IntervalType, duration: Duration, interval_number: usize) {
    let mut remaining = duration.as_secs();

    while remaining > 0 {
        // Update the countdown displays to terminal each second
        println!(
            "\r{} Interval #{}: {}s remaining",
            match interval_type {
                IntervalType::Work => "Work",
                IntervalType::Break => "Break",
            },
            interval_number,
            remaining
        );
        // Check for pause input
        if let Some('p') = check_for_input() {
            println!("Timer Paused. Press 'r' followed by Enter to resume");
            wait_for_resume();
        }
        // sleep for 1 second and update 'remaining'
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::thread::sleep(Duration::from_secs(1));
        remaining -= 1;
    }
    // Print message once interval has completed
    // Format message once interval completed
    let done_msg = match interval_type {
        IntervalType::Work => {
            format!("\rInterval #{} done  \x1B[K", interval_number)
        }
        IntervalType::Break => format!("\rBreak Done \x1B[K"),
    };
    println!("{}", done_msg);
}

// Monitor user input, when user enters "p" followed by Enter signal to calling function to pause the timer
fn check_for_input() -> Option<char> {
    let mut buffer = String::new();
    println!("Press 'p' to pause followed by 'Enter'");
    if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if buffer.trim() == "p" {
            return Some('p');
        }
    }
    return None;
}

// stop the program's execution until user to presses 'r' followed by enter
fn wait_for_resume() {
    let mut buffer = String::new();
    println!("Timer paused. Press 'r' then 'Enter' to resume");
    loop {
        if let Ok(_) = std::io::stdin().read_line(&mut buffer) {
            if buffer.trim() == "r" {
                println!("Timer Resumed");
                break;
            }
            buffer.clear();
        }
    }
}

#[allow(unused)]
pub fn test_check_for_input() {
    loop {
        match check_for_input() {
            Some('p') => {
                println!("Paused");
                wait_for_resume();
                break;
            }
            _ => {
                println!("Waiting for 'p' to pause...");
            }
        }
    }
}
