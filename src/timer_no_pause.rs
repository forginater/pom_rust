use std::io::Write;
use std::time::Duration; // Trait needs to be in scope use stdout.flush().... ??

/*
    Timer Logic
*/

enum IntervalType {
    Work,
    Break,
}

// Run the timer alternating between num_intervals of interval_len seconds and break intervals for break_len seconds.
pub fn timer_logic(
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

// For a given interval
fn countdown(interval_type: IntervalType, interval_duration: Duration, interval_number: usize) {
    let done_msg = match interval_type {
        IntervalType::Work => {
            format!("\rInterval #{} done  \x1B[K", interval_number)
        }
        IntervalType::Break => format!("\rBreak Done \x1B[K"),
    };

    // loop for each second of the interval (counting down to 0)
    for remaining in (0..=interval_duration.as_secs()).rev() {
        let init_msg = match interval_type {
            IntervalType::Work => {
                format!("\rInterval #{}: {}s remaining", interval_number, remaining)
            }
            IntervalType::Break => format!("\rBreak Time: {}s remaining", remaining),
        };

        print!("{}", init_msg);
        std::io::stdout().flush().expect("Failed to flush stdout");
        std::thread::sleep(Duration::from_secs(1));
    }

    println!("{}", done_msg);
}
