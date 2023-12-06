// Used to get the data
const DATA: &str = include_str!("../data/final/day06.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day06.txt");
    assert!(calc(DATA_PART1).0 == 288);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day06.txt");
    assert!(calc(DATA_PART2).1 == 71503);
}
fn ways_to_win(time: usize, distance: usize) -> usize {
    // Uses quadratic formula to solve (time-x)*x>=distance
    let distance = distance + 1;
    let max =
        ((time as f64 + ((time * time - 4 * distance) as f64).sqrt()) / 2_f64).floor() as usize;
    let min =
        ((time as f64 - ((time * time - 4 * distance) as f64).sqrt()) / 2_f64).ceil() as usize;
    max - min + 1
}
fn calc(data: &str) -> (usize, usize) {
    let mut lines = data.lines();
    let times: Vec<&str> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect();
    let distance: Vec<&str> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split(' ')
        .filter(|x| !x.is_empty())
        .collect();
    let mut total = 1;
    for i in 0..times.len() {
        let time = times[i].parse::<usize>().unwrap();
        let distance = distance[i].parse::<usize>().unwrap();
        total *= ways_to_win(time, distance);
    }
    (
        total,
        ways_to_win(
            times.concat().parse::<usize>().unwrap(),
            distance.concat().parse::<usize>().unwrap(),
        ),
    )
}
pub fn main() {
    let (answer1, answer2) = calc(DATA);
    println!("Answer for day 6 part 1 is {}.", answer1);
    println!("Answer for day 6 part 2 is {}.", answer2);
}
