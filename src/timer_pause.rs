use std::io::Write;
use std::time::Duration; // Trait needs to be in scope use stdout.flush().... ??

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
    let interval_seconds = work_interval_len.as_secs();
    println!("\nPomodoro Timer Started: {num_intervals} intervals of {interval_seconds} ");

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

fn countdown(interval_type: IntervalType, duration: Duration, interval_number: usize) {
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
