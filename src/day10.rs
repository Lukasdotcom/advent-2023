use std::collections::HashMap;

// Used to get the data
const DATA: &str = include_str!("../data/final/day10.txt");
#[test]
fn test_part1() {
    const DATA_PART1A: &str = include_str!("../data/test/day10a.txt");
    assert!(calc(DATA_PART1A).0 == 4);
    const DATA_PART1B: &str = include_str!("../data/test/day10b.txt");
    assert!(calc(DATA_PART1B).0 == 8);
}
#[derive(PartialEq, Eq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}
#[test]
fn test_part2() {
    const DATA_PARTC: &str = include_str!("../data/test/day10c.txt");
    assert!(calc(DATA_PARTC).1 == 4);
    const DATA_PARTD: &str = include_str!("../data/test/day10d.txt");
    assert!(calc(DATA_PARTD).1 == 4);
    const DATA_PARTE: &str = include_str!("../data/test/day10e.txt");
    assert!(calc(DATA_PARTE).1 == 10);
}
fn calc(data: &str) -> (usize, usize) {
    let mut directions: HashMap<char, [Direction; 2]> = HashMap::new();
    directions.insert('|', [Direction::North, Direction::South]);
    directions.insert('-', [Direction::East, Direction::West]);
    directions.insert('L', [Direction::North, Direction::East]);
    directions.insert('J', [Direction::West, Direction::North]);
    directions.insert('7', [Direction::South, Direction::West]);
    directions.insert('F', [Direction::East, Direction::South]);
    directions.insert('.', [Direction::None, Direction::None]);

    let map: Vec<Vec<char>> = data.split('\n').map(|x| x.chars().collect()).collect();
    let mut start: (usize, usize) = (0, 0);
    #[allow(clippy::needless_range_loop)]
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let char = &map[y][x];
            if *char == 'S' {
                start = (x, y);
            }
        }
    }
    let mut coords = vec![start];
    let mut current_direction: &Direction = &Direction::East;
    // Checks if you can move east
    if coords.len() <= 1 && start.0 + 1 < map[start.1].len() {
        let char = &map[start.1][start.0 + 1];
        if directions.get(char).unwrap().contains(&Direction::West) {
            current_direction = directions
                .get(char)
                .unwrap()
                .iter()
                .find(|x| *x != &Direction::West)
                .unwrap();
            coords.push((start.0 + 1, start.1));
        }
    }
    // Checks if you can move west
    if coords.len() <= 1 && start.0 > 0 {
        let char = &map[start.1][start.0 - 1];
        if directions.get(char).unwrap().contains(&Direction::East) {
            current_direction = directions
                .get(char)
                .unwrap()
                .iter()
                .find(|x| *x != &Direction::East)
                .unwrap();
            coords.push((start.0 - 1, start.1));
        }
    }
    // Checks if you can move south
    if coords.len() <= 1 && start.1 + 1 < map.len() {
        let char = &map[start.1 + 1][start.0];
        if directions.get(char).unwrap().contains(&Direction::North) {
            current_direction = directions
                .get(char)
                .unwrap()
                .iter()
                .find(|x| *x != &Direction::North)
                .unwrap();
            coords.push((start.0, start.1 + 1));
        }
    }
    // Checks if you can move north
    if coords.len() <= 1 && start.0 > 0 {
        let char = &map[start.1][start.0 - 1];
        if directions.get(char).unwrap().contains(&Direction::South) {
            current_direction = directions
                .get(char)
                .unwrap()
                .iter()
                .find(|x| *x != &Direction::South)
                .unwrap();
            coords.push((start.0, start.1 - 1));
        }
    }
    loop {
        // Does the next move
        let mut coord = coords[coords.len() - 1];
        let filter = match current_direction {
            Direction::East => {
                coord.0 += 1;
                &Direction::West
            }
            Direction::West => {
                coord.0 -= 1;
                &Direction::East
            }
            Direction::North => {
                coord.1 -= 1;
                &Direction::South
            }
            Direction::South => {
                coord.1 += 1;
                &Direction::North
            }
            Direction::None => {
                break;
            }
        };
        let char = &map[coord.1][coord.0];
        if char == &'S' {
            break;
        }
        coords.push(coord);
        current_direction = directions
            .get(char)
            .unwrap()
            .iter()
            .find(|x| *x != filter)
            .unwrap();
    }
    let mut count = 0;
    let mut map = map;
    map[start.1][start.0] = '|'; // Did not feel like finding out what the character for start should have been
    coords.sort();
    #[allow(clippy::needless_range_loop)]
    for y in 0..map.len() {
        let mut inside = 100000;
        for x in 0..map[y].len() {
            let contained = coords.binary_search(&(x, y)).is_ok();
            if contained && map[y][x] == '|' {
                inside += 2;
            }
            if contained && (map[y][x] == 'L' || map[y][x] == '7') {
                inside -= 1;
            }
            if contained && (map[y][x] == 'J' || map[y][x] == 'F') {
                inside += 1;
            }
            if !contained {
                map[y][x] = '.';
            }
            if !contained && (inside % 4) == 2 {
                map[y][x] = 'Z';
                count += 1;
            }
        }
    }
    (coords.len() / 2, count)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 10 part 1 is {}.", part1);
    println!("Answer for day 10 part 2 is {}.", part2);
}
