use std::time::Duration;

// TODO
// Dynamic logs (time remaining)
// prettify terminal output
// Add pause/resume functionality
// Add a break interval
// Add a work intended user input at beginning
// Add Prompt at end to check productivity, relevance etc
// write results to file or database

fn main() {
    // Pom takes user input (numIntervals, intervalLen) and runs a timer numIntervals times each for a length of intervalLen

    // Get user input (numIntervals)
    let num_intervals = get_num_intervals();

    // Get user input (intervalLen)
    let interval_len = get_interval_len();

    // Get planned activity
    let _activity = get_planned_action();

    // timer_logic(interval_len, num_intervals);
    timer_logic_dynamic(interval_len, num_intervals);
}

/*
    User Input
*/

// &str can only read, we don't have ownership, unlike String.
fn get_num_from_terminal(input_prompt: &str) -> usize {
    println!("{input_prompt}: ");

    let mut buffer = String::new();
    loop {
        // reference (rust concept) wraps a pointer with addtional info
        let result: Result<usize, std::io::Error> = std::io::stdin().read_line(&mut buffer);
        // Result used when could be runtime error
        match result {
            Err(_) => panic!("PANIC"),
            _ => {} // Any other enum variants, don't care.... "_" wildcard for variants
        };
        // validate input
        // - Need to trim the '\n' from the buffer string before parsing
        match buffer.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(e) => eprintln!("Try again dickhead: {e} \nbuffer: {buffer:?}"),
        };
        buffer.clear();
    }
}

fn get_string_from_terminal(input_prompt: &str) -> String {
    println!("{input_prompt}");

    let mut buffer = String::new();
    loop {
        match std::io::stdin().read_line(&mut buffer) {
            Err(_) => eprintln!("Error reading input: Try again"),
            Ok(_) => {
                // Check the string is valid
                let trimmed = buffer.trim();
                if !trimmed.is_empty() && trimmed.len() > 5 {
                    return trimmed.to_string(); // convert from &str to String
                } else {
                    eprintln!("Input cannot be empty: Try again {buffer:?}");
                }
            }
        }
        buffer.clear();
    }
}

fn get_num_intervals() -> usize {
    return get_num_from_terminal("Enter number of intervals");
}

fn get_interval_len() -> Duration {
    let interval_len_input = get_num_from_terminal("Enter length of each interval (in minutes)");
    return std::time::Duration::from_secs(interval_len_input as u64);
}

fn get_planned_action() -> String {
    return get_string_from_terminal("What activity will you work on?");
}

/*
    Timer Logic
*/

fn _timer_logic(interval_len: Duration, num_intervals: usize) {
    let mut intervals_done = 0;

    // Run the timer logic
    //  - Print timer start
    println!("\n\ntimer started");

    //  - If intervalsDone < numIntervals increment intervalsDone, otherwise run the timer again
    while intervals_done < num_intervals {
        //  - Run sleep for intervalLen
        std::thread::sleep(interval_len);
        intervals_done += 1;
        println!("interval #{intervals_done} done");
    }
    //  - Print all intervals done
    println!("all Done");
}

use std::io::Write; // Trait needs to be in scope use stdout.flush().... ??

fn timer_logic_dynamic(interval_len: Duration, num_intervals: usize) {
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

/*SCRQAPLANDLK */

// Contract with compiler, it cannot know which variant of the enum, just that it's an enum of type "Thing"
// Whereas it does know for struct
// enum Thing {
//     Cat,
//     Dog,
// }

// Alternate way to handle Result Enums
// if let Err(e) = std::io::stdout().flush() {
//     eprintln!("Failed to flush stdout: {}", e);
// }

// fn _timer_logic_dynamic_first(interval_len: Duration, num_intervals: usize) {
//     let mut intervals_done = 0;
//     let interval_seconds = interval_len.as_secs();

//     println!("\nPomodoro Timer Started: {num_intervals} intervals of {interval_seconds} ");

//     while intervals_done < num_intervals {
//         print!("Interval {}: ", intervals_done + 1);

//         // loop for each second of the interval (counting down to 0)
//         for remaining in (0..=interval_seconds).rev() {
//             // Print the countdown message
//             //  Note: carriage return '\r' moves cursor to beginning of line which allows us to overwrite
//             print!(
//                 "\rInterval #{}: {}s remaining",
//                 intervals_done + 1,
//                 remaining
//             );
//             // Flush output to terminal:
//             //  - Neccessary because Rust's stdout is line-buffered by default
//             //  - without flushing, output may not appear immediately
//             std::io::stdout().flush().unwrap();
//             // wait 1 second
//             std::thread::sleep(Duration::from_secs(1));
//         }
//         intervals_done += 1;
//         println!("\rInterval #{} done  \x1B[K", intervals_done);
//     }
//     println!("\nPomodoro completed");
// }
