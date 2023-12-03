// Used to get the data
const DATA: &str = include_str!("../data/final/day3.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day3.txt");
    assert!(calc(DATA_PART1).0 == 4361);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day3.txt");
    assert!(calc(DATA_PART2).1 == 467835);
}
fn neighbors(i: usize, j: usize, data: &[Vec<char>]) -> [(Option<&char>, (usize, usize)); 8] {
    [
        (
            data.get(if i > 0 { i - 1 } else { 2000 })
                .and_then(|x| x.get(if j > 0 { j - 1 } else { 2000 })),
            (
                if i > 0 { i - 1 } else { 2000 },
                if j > 0 { j - 1 } else { 2000 },
            ),
        ),
        (
            data.get(if i > 0 { i - 1 } else { 2000 })
                .and_then(|x| x.get(j)),
            (if i > 0 { i - 1 } else { 2000 }, j),
        ),
        (
            data.get(if i > 0 { i - 1 } else { 2000 })
                .and_then(|x| x.get(j + 1)),
            (if i > 0 { i - 1 } else { 2000 }, j + 1),
        ),
        (data.get(i + 1).and_then(|x| x.get(j + 1)), (i + 1, j + 1)),
        (data.get(i + 1).and_then(|x| x.get(j)), (i + 1, j)),
        (
            data.get(i + 1)
                .and_then(|x| x.get(if j > 0 { j - 1 } else { 2000 })),
            (i + 1, if j > 0 { j - 1 } else { 2000 }),
        ),
        (
            data.get(i)
                .and_then(|x| x.get(if j > 0 { j - 1 } else { 2000 })),
            (i, if j > 0 { j - 1 } else { 2000 }),
        ),
        (data.get(i).and_then(|x| x.get(j + 1)), (i, j + 1)),
    ]
}
fn valid_neighbor(i: usize, j: usize, data: &[Vec<char>]) -> bool {
    neighbors(i, j, data).iter().any(|(x, _)| match x {
        Some(x) => x != &&'.' && !x.is_ascii_digit(),
        None => false,
    })
}
fn calc(data: &str) -> (usize, usize) {
    let data: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();
    let mut part1_total = 0;
    let mut part2_total = 0;
    let mut valid_num = false;
    let mut curr_num = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == '*' {
                part2_total += count_neighbors(i, j, &data);
            } else {
                match data[i][j].to_digit(10) {
                    Some(x) => {
                        curr_num = curr_num * 10 + x as usize;
                        valid_num = valid_num || valid_neighbor(i, j, &data);
                    }
                    None => {
                        if valid_num {
                            part1_total += curr_num;
                        }
                        curr_num = 0;
                        valid_num = false;
                    }
                }
            }
        }
    }
    (part1_total, part2_total)
}
fn get_number(i: usize, j: usize, data: &[Vec<char>]) -> usize {
    let mut num: usize = data[i][j].to_digit(10).unwrap() as usize;
    let mut shift_left: usize = 1;
    while data[i][j - shift_left].is_ascii_digit() {
        num += data[i][j - shift_left].to_digit(10).unwrap() as usize
            * (10u32.pow(shift_left as u32) as usize);

        if j - shift_left == 0 {
            break;
        }
        shift_left += 1;
    }
    let mut j = j + 1;
    while j < data[i].len() && data[i][j].is_ascii_digit() {
        num *= 10;
        num += data[i][j].to_digit(10).unwrap() as usize;
        j += 1;
    }
    num
}
fn count_neighbors(i: usize, j: usize, data: &[Vec<char>]) -> usize {
    let og_iter = data;
    let mut data = neighbors(i, j, data);
    if data[1].0.and_then(|x| x.to_digit(10)).is_some() {
        if data[0].0.and_then(|x| x.to_digit(10)).is_some() {
            data[0].0 = None;
        }
        if data[2].0.and_then(|x| x.to_digit(10)).is_some() {
            data[2].0 = None;
        }
    }
    if data[4].0.and_then(|x| x.to_digit(10)).is_some() {
        if data[3].0.and_then(|x| x.to_digit(10)).is_some() {
            data[3].0 = None;
        }
        if data[5].0.and_then(|x| x.to_digit(10)).is_some() {
            data[5].0 = None;
        }
    }
    let mut num = 0;
    let a = data
        .iter()
        .filter(|(x, (_, _))| x.and_then(|x: &char| x.to_digit(10)).is_some())
        .map(|(_, (i, j))| {
            num += 1;
            get_number(i.to_owned(), j.to_owned(), og_iter)
        });
    let vec = a.collect::<Vec<usize>>();
    if vec.len() == 2 {
        vec.iter().product()
    } else {
        0
    }
}
pub fn main() {
    let (answer1, answer2) = calc(DATA);
    println!("Answer for day 1 part 1 is {}.", answer1);
    println!("Answer for day 1 part 2 is {}.", answer2);
}
