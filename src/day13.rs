// Used to get the data
const DATA: &str = include_str!("../data/final/day13.txt");
#[test]
fn test_part1() {
    const DATA_PART: &str = include_str!("../data/test/day13.txt");
    assert!(calc(DATA_PART).0 == 405);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day13.txt");
    assert!(calc(DATA_PART).1 == 400);
}
#[test]
fn find_reflections_test() {
    println!(
        "{:?}",
        find_reflections(".##..#...", (vec![1, 2, 3, 4, 5, 6, 7, 8], vec![]))
    );
    assert!(
        find_reflections(".##..#...", (vec![1, 2, 3, 4, 5, 6, 7, 8], vec![]))
            == (vec![2, 8], vec![1, 4, 6, 7])
    );
}
fn find_reflections(data: &str, valid: (Vec<usize>, Vec<usize>)) -> (Vec<usize>, Vec<usize>) {
    let mut result = (vec![], vec![]);
    for i in valid.0 {
        let remainder = match data.len() < i * 2 {
            true => i * 2 - data.len(),
            false => 0,
        };
        let mut diff = 0;
        for idx in 0..i - remainder {
            if data[remainder + idx..remainder + idx + 1]
                != data[i + i - remainder - idx - 1..i + i - remainder - idx]
            {
                diff += 1;
            }
            if diff > 2 {
                break;
            }
        }
        if diff == 0 {
            result.0.push(i);
        } else if diff == 1 {
            result.1.push(i);
        }
    }
    for i in valid.1 {
        let remainder = match data.len() < i * 2 {
            true => i * 2 - data.len(),
            false => 0,
        };
        let mut same = true;
        for idx in 0..i - remainder {
            if data[remainder + idx..remainder + idx + 1]
                != data[i + i - remainder - idx - 1..i + i - remainder - idx]
            {
                same = false;
                break;
            }
        }
        if same {
            result.1.push(i);
        }
    }
    result
}
fn calc(data: &str) -> (usize, usize) {
    let patterns = data.split("\n\n").map(|x| {
        let lines: Vec<&str> = x.lines().collect();
        // Finds a reflection over a column
        let mut valid_col: Vec<usize> = (1..lines[0].len()).collect();
        let mut valid2_col = vec![1];
        for line in &lines {
            (valid_col, valid2_col) = find_reflections(line, (valid_col, valid2_col));
            if valid_col.is_empty() && valid2_col.is_empty() {
                break;
            }
        }
        let mut valid_row: Vec<usize> = (1..lines.len()).collect();
        let mut valid2_row = vec![];
        if !valid_col.is_empty() && !valid2_col.is_empty() {
            return (*valid_col.first().unwrap(), *valid2_col.first().unwrap());
        }
        for i in 0..lines[0].len() {
            let new_str = lines
                .iter()
                .map(|x| x.chars().nth(i).unwrap())
                .collect::<String>();
            (valid_row, valid2_row) = find_reflections(&new_str, (valid_row, valid2_row));
            if (!valid_col.is_empty() || valid_row.is_empty())
                && (!valid2_col.is_empty() && valid2_row.is_empty())
            {
                break;
            }
        }
        (
            match valid_col.is_empty() {
                true => valid_row.first().unwrap() * 100,
                false => *valid_col.first().unwrap(),
            },
            match valid2_col.is_empty() {
                true => valid2_row.first().unwrap() * 100,
                false => *valid2_col.first().unwrap(),
            },
        )
    });
    let mut sum = (0, 0);
    for pattern in patterns {
        sum.0 += pattern.0;
        sum.1 += pattern.1;
    }
    sum
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 13 part 1 is {}.", part1);
    println!("Answer for day 13 part 2 is {}.", part2);
}
