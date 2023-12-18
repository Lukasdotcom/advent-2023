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
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
fn main() {
    let tasks: [fn(); 18] = [
        day01::main,
        day02::main,
        day03::main,
        day04::main,
        day05::main,
        day06::main,
        day07::main,
        day08::main,
        day09::main,
        day10::main,
        day11::main,
        day12::main,
        day13::main,
        day14::main,
        day15::main,
        day16::main,
        day17::main,
        day18::main,
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
