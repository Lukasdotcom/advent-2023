use std::collections::HashSet;

// Used to get the data
const DATA: &str = include_str!("../data/final/day04.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day04.txt");
    assert!(calc(DATA_PART1).0 == 13);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day04.txt");
    assert!(calc(DATA_PART2).1 == 30);
}
fn calc(data: &str) -> (usize, usize) {
    let data: Vec<(HashSet<usize>, HashSet<usize>)> = data
        .lines()
        .map(|x| {
            let x = x.split_once(':').unwrap().1;
            let (a, b) = x.split_once('|').unwrap();
            (
                a.trim()
                    .split(' ')
                    .filter_map(|x| match x.parse::<usize>() {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    })
                    .collect(),
                b.trim()
                    .split(' ')
                    .filter_map(|x| match x.parse::<usize>() {
                        Ok(x) => Some(x),
                        Err(_) => None,
                    })
                    .collect(),
            )
        })
        .collect();
    let totals: Vec<usize> = data
        .into_iter()
        .map(|(a, b)| a.intersection(&b).count())
        .collect::<Vec<usize>>();
    let mut totals2 = vec![1; totals.len()];
    for (i, x) in totals.iter().enumerate() {
        for x in 0..*x {
            totals2[i + 1 + x] += totals2[i];
        }
    }
    (
        totals
            .into_iter()
            .map(|x| {
                let mut total = 0;
                let mut count = x;
                while count > 0 {
                    count -= 1;
                    if total == 0 {
                        total = 1;
                    } else {
                        total *= 2;
                    }
                }
                total
            })
            .sum(),
        totals2.iter().sum(),
    )
}
pub fn main() {
    let (answer1, answer2) = calc(DATA);
    println!("Answer for day 4 part 1 is {}.", answer1);
    println!("Answer for day 4 part 2 is {}.", answer2);
}
