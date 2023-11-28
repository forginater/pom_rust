use std::time::Duration; // Trait needs to be in scope use stdout.flush().... ??
                         /*
                             Get User Input to configure pomodoro timer
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
                if !trimmed.is_empty() {
                    return trimmed.to_string(); // convert from &str to String
                } else {
                    eprintln!("Input cannot be empty: Try again {buffer:?}");
                }
            }
        }
        buffer.clear();
    }
}

pub fn get_num_intervals() -> usize {
    return get_num_from_terminal("Enter number of intervals");
}

pub fn get_work_interval_len() -> Duration {
    let interval_len_input = get_num_from_terminal("Enter length of each interval (in minutes)");
    return std::time::Duration::from_secs(interval_len_input as u64);
}

pub fn get_break_interval_len() -> Duration {
    let break_interval = get_num_from_terminal("Enter length of break interval, or 0 if no breaks");
    return std::time::Duration::from_secs(break_interval as u64);
}

pub fn get_planned_action() -> String {
    return get_string_from_terminal("What activity will you work on?");
}
