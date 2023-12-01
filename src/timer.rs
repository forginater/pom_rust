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

enum EventOutcome {
    PauseToggled,
    Exit,
    Continue,
}

enum TimeFormat {
    Verbose,
    Compact,
}

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

// Run the timer alternating between num_intervals of interval_len seconds and break intervals for break_len seconds.
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
    display_finish_pom(num_intervals, work_interval_len);
}

fn interval_countdown(interval_type: IntervalType, duration: Duration, interval_number: usize) {
    let mut remaining: u64 = duration.as_secs();
    let mut is_paused: bool = false;
    while remaining > 0 {
        // Poll for keyboard events: pause/resume and exit
        match handle_events() {
            EventOutcome::PauseToggled => {
                // Handle "p" for pause/resume
                is_paused = !is_paused;
                display_pause_toggled(is_paused);
            }
            EventOutcome::Exit => {
                // Handle Ctrl+C to exit
                println!("\n\rExiting...\r");
                terminal::disable_raw_mode().expect("Failed to disable raw mode");
                std::process::exit(0);
            }
            EventOutcome::Continue => {}
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

// handle_events
//  - Non-blocking check for user input to pause the timer
//  - If an event is detected within 50ms, poll returns true
//  - NOTE: this runs approximately once per loop iteration, however keyboard input during the "sleep" duration will is queued in system's input buffer
fn handle_events() -> EventOutcome {
    if event::poll(Duration::from_millis(1)).unwrap() {
        let input_event = event::read().unwrap();
        match input_event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => {
                match code {
                    KeyCode::Char('p') => {
                        return EventOutcome::PauseToggled; // Indicate pause/resume
                    }
                    KeyCode::Char('c') if modifiers.contains(event::KeyModifiers::CONTROL) => {
                        return EventOutcome::Exit; // Exit Program
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
    EventOutcome::Continue
}

/*
    Functions to print the countdown state:
        "\r" = carriage return
        "\x1B[K" clears the line
        "\x1B[A" move cursor to previous line
*/

fn display_pause_toggled(is_paused: bool) {
    if is_paused {
        print!("\n\rPaused. Press 'p' again to resume");
    } else {
        print!("\r \x1B[K"); // Clear current line
        print!("\x1B[A\x1B[K"); // Clear preceding line
    }
    io::stdout().flush().unwrap();
}

fn display_countdown(interval_type: &IntervalType, interval_number: usize, remaining: u64) {
    let interval_label = match interval_type {
        IntervalType::Work => "Work",
        IntervalType::Break => "Break",
    };
    let remaining_formatted: String =
        format_time(Duration::from_secs(remaining), TimeFormat::Compact);
    print!(
        "\r{} Interval #{}: remaining = {}",
        interval_label, interval_number, remaining_formatted
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

fn display_finish_pom(num_intervals: usize, work_interval_len: Duration) {
    // Calculate total time working as a Duration
    // ??????
    let total_work_duration = if let Ok(multiplier) = u32::try_from(num_intervals) {
        work_interval_len
            .checked_mul(multiplier) // Return Some(Duration) or None (if overflow)
            .unwrap_or_else(|| Duration::new(0, 0))
    } else {
        // Handle potential overflow or conversion error here
        Duration::new(0, 0)
    };

    println!(
        "\n\r@{}: Pomodoro completed: \n\r\tTotal time working = {}\r",
        get_now(),
        format_time(total_work_duration, TimeFormat::Verbose)
    );
}

// Convert a Duration value to a formatted string eg "1h 2m 2s"
fn format_time(duration_arg: Duration, format: TimeFormat) -> String {
    let total_seconds = duration_arg.as_secs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    match format {
        TimeFormat::Verbose => {
            let mut duration_str = String::new();

            if hours > 0 {
                duration_str.push_str(&format!("{}h ", hours));
            }
            if minutes > 0 {
                duration_str.push_str(&format!("{}m ", minutes));
            }
            if seconds > 0 || duration_str.is_empty() {
                duration_str.push_str(&format!("{}s", seconds));
            }
            duration_str
        }
        TimeFormat::Compact => {
            if hours > 0 {
                return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
            } else if minutes > 0 {
                return format!("{:02}:{:02}", minutes, seconds);
            } else {
                return format!("{} seconds", seconds);
            }
        }
    }
}

fn get_now() -> DelayedFormat<StrftimeItems<'static>> {
    return Local::now().format("%H:%M");
}
