use std::collections::HashSet;

// Used to get the data
const DATA: &str = include_str!("../data/final/day16.txt");
#[test]
fn test_part1() {
    const DATA_PART: &str = include_str!("../data/test/day16.txt");
    assert!(calc(DATA_PART).0 == 46);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day16.txt");
    assert!(calc(DATA_PART).1 == 51);
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}
fn next_position(position: Position) -> Position {
    let mut position = position;
    match position.direction {
        Direction::Up => position.y -= 1,
        Direction::Down => position.y += 1,
        Direction::Left => position.x -= 1,
        Direction::Right => position.x += 1,
    }
    position
}
fn travel_path(
    map: &Vec<Vec<char>>,
    position: Position,
    visited: HashSet<Position>,
) -> HashSet<Position> {
    if visited.contains(&position) {
        return visited;
    }
    if position.y > map.len() as i32 - 1 {
        return visited;
    }
    if position.x > map[0].len() as i32 - 1 {
        return visited;
    }
    if position.x < 0 || position.y < 0 {
        return visited;
    }
    let mut visited = visited;
    visited.insert(position);
    match map[position.y as usize][position.x as usize] {
        '.' => travel_path(map, next_position(position), visited),
        '|' => match position.direction {
            Direction::Up | Direction::Down => travel_path(map, next_position(position), visited),
            Direction::Left | Direction::Right => {
                let position_down = Position {
                    x: position.x,
                    y: position.y,
                    direction: Direction::Down,
                };
                let mut position_up = position;
                position_up.direction = Direction::Up;
                let visted = travel_path(map, next_position(position_down), visited);

                travel_path(map, next_position(position_up), visted)
            }
        },
        '-' => match position.direction {
            Direction::Left | Direction::Right => {
                travel_path(map, next_position(position), visited)
            }
            Direction::Down | Direction::Up => {
                let position_right = Position {
                    x: position.x,
                    y: position.y,
                    direction: Direction::Right,
                };
                let mut position_left = position;
                position_left.direction = Direction::Left;
                let visted = travel_path(map, next_position(position_right), visited);
                travel_path(map, next_position(position_left), visted)
            }
        },
        '/' => {
            let mut position = position;
            match position.direction {
                Direction::Up => position.direction = Direction::Right,
                Direction::Down => position.direction = Direction::Left,
                Direction::Left => position.direction = Direction::Down,
                Direction::Right => position.direction = Direction::Up,
            };
            travel_path(map, next_position(position), visited)
        }
        '\\' => {
            let mut position = position;
            match position.direction {
                Direction::Up => position.direction = Direction::Left,
                Direction::Down => position.direction = Direction::Right,
                Direction::Left => position.direction = Direction::Up,
                Direction::Right => position.direction = Direction::Down,
            };
            travel_path(map, next_position(position), visited)
        }
        _ => {
            panic!("Invalid Character")
        }
    }
}
fn add_to_edges(
    position: &Position,
    edges_visited: &mut HashSet<Position>,
    max_x: i32,
    max_y: i32,
) {
    if position.x == 0 && position.direction == Direction::Left {
        edges_visited.insert(Position {
            x: position.x,
            y: position.y,
            direction: Direction::Right,
        });
    } else if position.x == max_x && position.direction == Direction::Right {
        edges_visited.insert(Position {
            x: position.x,
            y: position.y,
            direction: Direction::Left,
        });
    } else if position.y == 0 && position.direction == Direction::Up {
        edges_visited.insert(Position {
            x: position.x,
            y: position.y,
            direction: Direction::Down,
        });
    } else if position.y == max_y && position.direction == Direction::Down {
        edges_visited.insert(Position {
            x: position.x,
            y: position.y,
            direction: Direction::Up,
        });
    }
}
fn calc(data: &str) -> (usize, usize) {
    let map: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();
    let mut part1 = 0;
    let mut part2 = part1;
    let mut edges_visited: HashSet<Position> = HashSet::new();
    for y in 0..map.len() {
        let position = Position {
            x: 0,
            y: y as i32,
            direction: Direction::Right,
        };
        if !edges_visited.contains(&position) {
            let visited: HashSet<Position> = HashSet::new();
            let visited = travel_path(&map, position, visited);
            let visited: HashSet<(i32, i32)> = visited
                .iter()
                .map(|x| {
                    add_to_edges(
                        x,
                        &mut edges_visited,
                        map[0].len() as i32 - 1,
                        map.len() as i32 - 1,
                    );
                    (x.x, x.y)
                })
                .collect();
            let maybe_part2 = visited.len();
            if y == 0 {
                part1 = maybe_part2;
            }
            if maybe_part2 > part2 {
                part2 = maybe_part2;
            }
        }
        let position = Position {
            x: map[0].len() as i32 - 1,
            y: y as i32,
            direction: Direction::Left,
        };
        if !edges_visited.contains(&position) {
            let visited: HashSet<Position> = HashSet::new();
            let visited = travel_path(&map, position, visited);
            let visited: HashSet<(i32, i32)> = visited
                .iter()
                .map(|x| {
                    add_to_edges(
                        x,
                        &mut edges_visited,
                        map[0].len() as i32 - 1,
                        map.len() as i32 - 1,
                    );
                    (x.x, x.y)
                })
                .collect();
            let maybe_part2 = visited.len();
            if maybe_part2 > part2 {
                part2 = maybe_part2;
            }
        }
    }
    for x in 0..map.len() {
        let position = Position {
            x: x as i32,
            y: 0,
            direction: Direction::Down,
        };
        if !edges_visited.contains(&position) {
            let visited: HashSet<Position> = HashSet::new();
            let visited = travel_path(&map, position, visited);
            let visited: HashSet<(i32, i32)> = visited
                .iter()
                .map(|x| {
                    add_to_edges(
                        x,
                        &mut edges_visited,
                        map[0].len() as i32 - 1,
                        map.len() as i32 - 1,
                    );
                    (x.x, x.y)
                })
                .collect();
            let maybe_part2 = visited.len();
            if maybe_part2 > part2 {
                part2 = maybe_part2;
            }
        }
        let position = Position {
            x: x as i32,
            y: map.len() as i32 - 1,
            direction: Direction::Up,
        };
        if !edges_visited.contains(&position) {
            let visited: HashSet<Position> = HashSet::new();
            let visited = travel_path(&map, position, visited);
            let visited: HashSet<(i32, i32)> = visited
                .iter()
                .map(|x| {
                    add_to_edges(
                        x,
                        &mut edges_visited,
                        map[0].len() as i32 - 1,
                        map.len() as i32 - 1,
                    );
                    (x.x, x.y)
                })
                .collect();
            let maybe_part2 = visited.len();
            if maybe_part2 > part2 {
                part2 = maybe_part2;
            }
        }
    }
    (part1, part2)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 16 part 1 is {}.", part1);
    println!("Answer for day 16 part 2 is {}.", part2);
}
