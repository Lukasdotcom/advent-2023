// Used to get the data
const DATA: &str = include_str!("../data/final/day2.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day2_part1.txt");
    assert!(calc(DATA_PART1).0 == 8);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day2_part2.txt");
    assert!(calc(DATA_PART2).1 == 2286);
}
fn valid_line(line: &str) -> (bool, (usize, usize, usize)) {
    let mut result = true;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for part in line.split(',') {
        if part.contains("red") {
            red = part
                .get(..part.len() - 3)
                .unwrap_or("a")
                .parse::<usize>()
                .unwrap_or(100);
            if red > MAX_RED {
                result = false;
            }
        } else if part.contains("green") {
            green = part
                .get(..part.len() - 5)
                .unwrap_or("a")
                .parse::<usize>()
                .unwrap_or(100);
            if green > MAX_GREEN {
                result = false;
            }
        } else if part.contains("blue") {
            blue = part
                .get(..part.len() - 4)
                .unwrap_or("a")
                .parse::<usize>()
                .unwrap_or(100);
            if blue > MAX_BLUE {
                result = false;
            }
        }
    }
    (result, (red, green, blue))
}
const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;
pub fn calc(data: &str) -> (usize, usize) {
    let mut total_part1 = 0;
    let mut total_part2 = 0;
    let data = data.replace(' ', "");
    for line in data.lines() {
        let id = line
            .get(
                line.find("Game").expect("No game id found") + 4
                    ..line.find(':').expect("No colon found"),
            )
            .expect("No id found")
            .parse::<usize>()
            .unwrap_or(0);
        let line = line
            .get(line.find(':').expect("No colon found") + 1..)
            .expect("No line found");
        let mut valid = true;
        let mut big_red = 0;
        let mut big_green = 0;
        let mut big_blue = 0;
        for part in line.split(';') {
            let (is_valid, (red, green, blue)) = valid_line(part);
            if !is_valid {
                valid = false;
            }
            if red > big_red {
                big_red = red;
            }
            if green > big_green {
                big_green = green;
            }
            if blue > big_blue {
                big_blue = blue;
            }
        }
        if valid {
            total_part1 += id;
        }
        total_part2 += big_red * big_green * big_blue;
    }
    (total_part1, total_part2)
}
pub fn main() {
    let (answer1, answer2) = calc(DATA);
    println!("Answer for day 1 part 1 is {}.", answer1);
    println!("Answer for day 1 part 2 is {}.", answer2);
}
