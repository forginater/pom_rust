use std::time::Duration;

fn main() {
    // Pom takes user input (numIntervals, intervalLen) and runs a timer numIntervals times each for a length of intervalLen

    // Get user input (numIntervals)
    let num_intervals = get_num_intervals();

    // Get user input (intervalLen)
    let interval_len = get_interval_len();

    timer_logic(interval_len, num_intervals);
}

fn get_num_from_terminal(input_prompt: &str) -> usize {
    println!("{input_prompt}");

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
        match buffer.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(e) => println!("Try again dickhead:: {e} \nbuffer: {buffer:?}"),
        };
        buffer.clear();
    }
}

fn get_num_intervals() -> usize {
    return get_num_from_terminal("Enter number of intervals");
}

fn get_interval_len() -> Duration {
    let interval_len_input = get_num_from_terminal("Enter length of interval");
    return std::time::Duration::from_secs(interval_len_input as u64);
}

fn timer_logic(interval_len: Duration, num_intervals: usize) {
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

/*SCRQAPLANDLK */

// Contract with compiler, it cannot know which variant of the enum, just that it's an enum of type "Thing"
// Whereas it does know for struct
// enum Thing {
//     Cat,
//     Dog,
// }
