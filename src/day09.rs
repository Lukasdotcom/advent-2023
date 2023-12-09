// Used to get the data
const DATA: &str = include_str!("../data/final/day09.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day09.txt");
    assert!(calc(DATA_PART1).0 == 114);
}
#[test]
fn test_calc_next_digit() {
    assert!(calc_next_digit(vec![0, 3, 6, 9, 12, 15]).0 == 18);
    assert!(calc_next_digit(vec![0, 3, 6, 9, 12, 15]).1 == -3);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day09.txt");
    assert!(calc(DATA_PART2).1 == 2);
}
fn calc_next_digit(nums: Vec<i64>) -> (i64, i64) {
    let mut nums: Vec<Vec<i64>> = vec![nums];
    while nums.last().unwrap().iter().any(|x| *x != 0) {
        let mut new: Vec<i64> = vec![];
        let last = nums.last().unwrap();
        for i in 1..last.len() {
            new.push(last[i] - last[i - 1]);
        }
        nums.push(new);
    }
    let mut part1 = 0;
    let mut part2 = 0;
    for num in nums.iter().rev() {
        part1 += num.last().unwrap();
        part2 = num.first().unwrap() - part2;
    }
    (part1, part2)
}
fn calc(data: &str) -> (i64, i64) {
    let calculations: Vec<(i64, i64)> = data
        .lines()
        .map(|x| calc_next_digit(x.split(' ').map(|x| x.parse::<i64>().unwrap()).collect()))
        .collect();
    (
        calculations.iter().map(|x| x.0).sum(),
        calculations.iter().map(|x| x.1).sum(),
    )
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 9 part 1 is {}.", part1);
    println!("Answer for day 9 part 2 is {}.", part2);
}
