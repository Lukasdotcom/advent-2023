use std::time::Instant;
// The day to show (0 means all of them)
const DAY: u32 = 0;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
fn main() {
    let tasks: [fn(); 8] = [
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
        day08::main,
    ];
    let now = Instant::now();
    if DAY == 0 {
        for task in tasks {
            task();
        }
        let elapsed_time = now.elapsed().as_micros();
        println!("All the days took {elapsed_time} µs")
    } else {
        #[allow(arithmetic_overflow)]
        tasks[DAY as usize - 1]();
        let elapsed_time = now.elapsed().as_micros();
        println!("Day {DAY} took {elapsed_time} µs")
    }
}
