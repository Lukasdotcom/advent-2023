const CYCLES: usize = 1000000000;
// Used to get the data
const DATA: &str = include_str!("../data/final/day14.txt");
#[test]
fn test_part1() {
    const DATA_PART: &str = include_str!("../data/test/day14.txt");
    assert!(calc(DATA_PART).0 == 136);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day14.txt");
    assert!(calc(DATA_PART).1 == 64);
}
// Note that this also rotates the map
fn push_north(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; data[0].len()]; data.len()];
    // Goes through every column
    for x in 0..data[0].len() {
        let mut amount = 0;
        for y in (0..data.len()).rev() {
            match data[y][x] {
                'O' => {
                    result[y][x] = '.';
                    amount += 1;
                }
                '#' => {
                    result[y][x] = '#';
                    #[allow(clippy::needless_range_loop)]
                    for y in y + 1..y + 1 + amount {
                        result[y][x] = 'O';
                    }
                    amount = 0;
                }
                _ => (),
            }
        }
        #[allow(clippy::needless_range_loop)]
        for y in 0..amount {
            result[y][x] = 'O';
        }
    }
    result
}
fn push_south(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; data[0].len()]; data.len()];
    // Goes through every column
    for x in 0..data[0].len() {
        let mut amount = 0;
        for y in 0..data.len() {
            match data[y][x] {
                'O' => {
                    result[y][x] = '.';
                    amount += 1;
                }
                '#' => {
                    result[y][x] = '#';
                    #[allow(clippy::needless_range_loop)]
                    if amount > 0 {
                        for y in y - amount..=(y - 1) {
                            result[y][x] = 'O';
                        }
                        amount = 0;
                    }
                }
                _ => (),
            }
        }
        #[allow(clippy::needless_range_loop)]
        for y in data.len() - amount..data.len() {
            result[y][x] = 'O';
        }
    }
    result
}
fn push_west(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; data[0].len()]; data.len()];
    // Goes through every column
    for y in 0..data[0].len() {
        let mut amount = 0;
        for x in (0..data.len()).rev() {
            match data[y][x] {
                'O' => {
                    result[y][x] = '.';
                    amount += 1;
                }
                '#' => {
                    result[y][x] = '#';
                    #[allow(clippy::needless_range_loop)]
                    for x in x + 1..x + 1 + amount {
                        result[y][x] = 'O';
                    }
                    amount = 0;
                }
                _ => (),
            }
        }
        #[allow(clippy::needless_range_loop)]
        for x in 0..amount {
            result[y][x] = 'O';
        }
    }
    result
}
fn push_east(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; data[0].len()]; data.len()];
    // Goes through every column
    for y in 0..data[0].len() {
        let mut amount = 0;
        for x in 0..data.len() {
            match data[y][x] {
                'O' => {
                    result[y][x] = '.';
                    amount += 1;
                }
                '#' => {
                    result[y][x] = '#';
                    #[allow(clippy::needless_range_loop)]
                    if amount > 0 {
                        for x in x - amount..=(x - 1) {
                            result[y][x] = 'O';
                        }
                        amount = 0;
                    }
                }
                _ => (),
            }
        }
        #[allow(clippy::needless_range_loop)]
        for x in data.len() - amount..data.len() {
            result[y][x] = 'O';
        }
    }
    result
}
fn calc(data: &str) -> (usize, usize) {
    let data = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut part1 = 0;
    let mut cache: Vec<Vec<Vec<char>>> = vec![];
    cache.push(data.clone());
    let data = push_north(data);
    for line in 0..data.len() {
        let increase = data.len() - line;
        part1 += data[line].iter().filter(|c| **c == 'O').count() * increase;
    }
    let mut data = push_east(push_south(push_west(data)));
    if cache.contains(&data) {
        return (part1, part1);
    }
    cache.push(data.clone());
    for count in 2..=CYCLES {
        data = push_east(push_south(push_west(push_north(data))));
        if let Some(start) = cache.iter().position(|x| x == &data) {
            let final_count = (CYCLES - count) % (count - start) + start;
            data = cache[final_count].clone();
            break;
        }
        cache.push(data.clone());
    }
    let mut part2 = 0;
    for line in 0..data.len() {
        let increase = data.len() - line;
        part2 += data[line].iter().filter(|c| **c == 'O').count() * increase;
    }
    (part1, part2)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 14 part 1 is {}.", part1);
    println!("Answer for day 14 part 2 is {}.", part2);
}
