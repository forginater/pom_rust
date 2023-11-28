/*Scrap Code */

// For a given interval
fn _countdown_no_pause(
    interval_type: IntervalType,
    interval_duration: Duration,
    interval_number: usize,
) {
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

// Original implementation (before adding dynamic messaging to terminal)
fn _timer_logic_dynamic(interval_len: Duration, num_intervals: usize) {
    let interval_seconds = interval_len.as_secs();

    println!("\nPomodoro Timer Started: {num_intervals} intervals of {interval_seconds} ");

    // loop through each interval
    for interval in 1..=num_intervals {
        // loop for each second of the interval (counting down to 0)
        for remaining in (0..=interval_seconds).rev() {
            // Print the countdown message
            //  Note: carriage return '\r' moves cursor to beginning of line which allows us to overwrite
            print!("\rInterval #{}: {}s remaining", interval, remaining);
            // Flush output to terminal:
            //  - Neccessary because Rust's stdout is line-buffered by default
            //  - Without flushing, output may not appear immediately
            std::io::stdout().flush().expect("Failed to flush stdout");
            // wait 1 second
            std::thread::sleep(Duration::from_secs(1));
        }
        // replace the line with done message:
        //  - Note the ANSI escape sequence \x1B[K clears rest of line
        println!("\rInterval #{} done  \x1B[K", interval);
    }
    println!("\nPomodoro completed");
}

// Alternate way to handle Result Enums
// if let Err(e) = std::io::stdout().flush() {
//     eprintln!("Failed to flush stdout: {}", e);
// }
