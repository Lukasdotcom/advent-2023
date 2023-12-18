// Used to get the data
const DATA: &str = include_str!("../data/final/day18.txt");
#[test]
fn test_part1() {
    const DATA_PART: &str = include_str!("../data/test/day18.txt");
    assert!(calc(DATA_PART).0 == 62);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day18.txt");
    assert!(calc(DATA_PART).1 == 952408144115);
}
struct Mined {
    coords: Vec<(i64, i64)>,
    perimeter: i64,
}
impl Mined {
    fn new() -> Mined {
        Mined {
            coords: vec![],
            perimeter: 0,
        }
    }
    fn command(&mut self, direction: char, amount: i64) {
        let mut old_coord = *self.coords.last().unwrap_or(&(0, 0));
        self.perimeter += amount;
        match direction {
            'U' | '3' => {
                old_coord.1 += amount;
            }
            'D' | '1' => {
                old_coord.1 -= amount;
            }
            'L' | '2' => {
                old_coord.0 -= amount;
            }
            'R' | '0' => {
                old_coord.0 += amount;
            }
            x => {
                panic!("Unknown direction {}", x);
            }
        };
        self.coords.push(old_coord);
    }
    fn area(&self) -> i64 {
        let mut result = 0;
        let coords = &self.coords;
        for y in 0..coords.len() {
            let coords_1 = coords[y];
            let coords_2 = if y == coords.len() - 1 {
                coords[0]
            } else {
                coords[y + 1]
            };
            result += coords_1.1 * coords_2.0 - coords_1.0 * coords_2.1;
        }
        result / 2 + self.perimeter / 2 + 1
    }
}
fn calc(data: &str) -> (i64, i64) {
    let mut mined_part1: Mined = Mined::new();
    let mut mined_part2: Mined = Mined::new();
    data.lines().for_each(|line| {
        let mut parts = line.split(' ');
        mined_part1.command(
            parts.next().unwrap().chars().next().unwrap(),
            parts.next().unwrap().parse().unwrap(),
        );
        let color = parts.next().unwrap();

        let color = &color[2..color.len() - 1];
        let direction = color.chars().last().unwrap();
        let amount = i64::from_str_radix(&color[..color.len() - 1], 16).unwrap();
        mined_part2.command(direction, amount);
    });
    (mined_part1.area(), mined_part2.area())
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 18 part 1 is {}.", part1);
    println!("Answer for day 18 part 2 is {}.", part2);
}
