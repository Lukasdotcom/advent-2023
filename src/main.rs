use std::time::Instant;
// The day to show (0 means all of them)
const DAY: u32 = 0;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
fn main() {
    let now = Instant::now();
    if DAY == 0 || DAY == 1 {
        day01::main();
    }
    if DAY == 0 || DAY == 2 {
        day02::main();
    }
    if DAY == 0 || DAY == 3 {
        day03::main();
    }
    if DAY == 0 || DAY == 4 {
        day04::main();
    }
    if DAY == 0 || DAY == 5 {
        day05::main();
    }
    if DAY == 0 || DAY == 6 {
        day06::main();
    }
    let elapsed_time = now.elapsed().as_micros();
    if DAY == 0 {
        println!("All the days took {elapsed_time} µs")
    } else {
        println!("Day {DAY} took {elapsed_time} µs")
    }
}
