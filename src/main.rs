use std::time::{SystemTime, UNIX_EPOCH};
use text_io::read;


fn main() {
    println!("Countdown Speed Test. Rust // Cargo Compiled Edition\nBy Cain Atkinson\nPress enter to run test. Big O calculated by me to be roughly O(0.3n)");
    let line: String = read!("{}\n");

    println!("Running tests.");
    // Tests go down here
    let _80k:f64 = count_down_time(80_000);
    println!("12.5%");

    let _800k:f64 = count_down_time(800_000);
    println!("25%");

    let _8m:f64 = count_down_time(8_000_000);
    println!("37.5%");

    let _80m:f64 = count_down_time(80_000_000);
    println!("50%");

    let _150m:f64 = count_down_time(150_000_000);
    println!("62.5%");

    let _300m:f64 = count_down_time(300_000_000);
    println!("75%");

    let _1b:f64 = count_down_time(1_000_000_000);
    println!("87.5%");

    let _10b:f64 = count_down_time(10_000_000_000);
    println!("100%");
    // Tests go up there

    println!("\nAll tests done.\n");

    println!("\
80 thousand test:  {} seconds\n\
800 thousand test: {} seconds\n\
8 million test:    {} seconds\n\
80 million test:   {} seconds\n\
150 million test:  {} seconds\n\
300 million test:  {} seconds\n\
1 billion test:    {} seconds\n\
10 billion test:   {} seconds",
             _80k, _800k, _8m, _80m, _150m, _300m, _1b, _10b);
    println!("Press Enter // Return to Exit.");
    let line: String = read!("{}\n");
}

// Get amount of time since the Unix Epoch
fn get_seconds_since_epoch() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_secs_f64();
}
// CountDown
fn count_down(n:u64) {
    let mut counter = n;
    while counter > 0 {
        counter -= 1;
    }
}
// Get amount of time taken to CountDown
fn count_down_time(n:u64) -> f64 {
    let start_time = get_seconds_since_epoch();
    count_down(n);
    let end_time = get_seconds_since_epoch();
    return end_time - start_time;
}