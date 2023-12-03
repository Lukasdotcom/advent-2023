use std::time::Instant;
// The day to show (0 means all of them)
const DAY: u32 = 0;
mod day1;
mod day2;
mod day3;
fn main() {
    let now = Instant::now();
    if DAY == 0 || DAY == 1 {
        day1::main();
    }
    if DAY == 0 || DAY == 2 {
        day2::main();
    }
    if DAY == 0 || DAY == 3 {
        day3::main();
    }
    let elapsed_time = now.elapsed().as_micros();
    if DAY == 0 {
        println!("All the days took {elapsed_time} µs")
    } else {
        println!("Day {DAY} took {elapsed_time} µs")
    }
}
