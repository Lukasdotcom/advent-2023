use std::time::{Duration, Instant};
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
mod day19;
mod day20;
mod day21;
mod day22;
fn main() {
    let tasks: [fn(); 22] = [
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
        day19::main,
        day20::main,
        day21::main,
        day22::main,
    ];
    let now = Instant::now();
    if DAY == 0 {
        for task in tasks {
            task();
        }
        print_data("All the days", now.elapsed());
    } else {
        #[allow(arithmetic_overflow)]
        tasks[DAY as usize - 1]();
        print_data(&format!("Day {DAY}"), now.elapsed());
    }
}
fn print_data(data: &str, time: Duration) {
    let time = time.as_nanos() as f32;
    let digits = (time).log10().ceil() as usize;
    match digits {
        0..=3 => println!("{}: {} ns", data, time),
        4 => println!("{}: {:.3} μs", data, time / 1_000.0),
        5 => println!("{}: {:.2} μs", data, time / 1_000.0),
        6 => println!("{}: {:.1} μs", data, time / 1_000.0),
        7 => println!("{}: {:.3} ms", data, time / 1_000_000.0),
        8 => println!("{}: {:.2} ms", data, time / 1_000_000.0),
        9 => println!("{}: {:.1} ms", data, time / 1_000_000.0),
        10 => println!("{}: {:.3} s", data, time / 1_000_000_000.0),
        11 => println!("{}: {:.2} s", data, time / 1_000_000_000.0),
        12 => println!("{}: {:.1} s", data, time / 1_000_000_000.0),
        _ => println!("{}: {:.0} s", data, time / 1_000_000_000.0),
    }
}
