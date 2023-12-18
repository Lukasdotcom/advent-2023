use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

// Used to get the data
const DATA: &str = include_str!("../data/final/day17.txt");
#[test]
fn test_part1() {
    const DATA_PART_SIMPLE: &str = include_str!("../data/test/day17_simple.txt");
    assert!(calc(DATA_PART_SIMPLE).0 == 8);
    const DATA_PART: &str = include_str!("../data/test/day17.txt");
    assert!(calc(DATA_PART).0 == 102);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day17.txt");
    assert!(calc(DATA_PART).1 == 94);
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    coords: (usize, usize),
    heat_cost: usize,
    direction: Direction,
    amount_in_direction: usize,
    max: (usize, usize),
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_cost.cmp(&self.heat_cost)
    }
}
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Position {
    // Gets all the neighbors of a position
    fn neighbors(
        &self,
        min_straight: usize,
        max_straight: usize,
        map: &[Vec<usize>],
    ) -> Vec<Position> {
        let mut result: Vec<Position> = vec![];
        if max_straight > self.amount_in_direction {
            if let Some(new_coords) = self.move_coords(self.direction, map) {
                result.push(new_coords);
            }
        }
        if min_straight <= self.amount_in_direction {
            match self.direction {
                Direction::Up | Direction::Down => {
                    if let Some(new_coords) = self.move_coords(Direction::Left, map) {
                        result.push(new_coords);
                    }
                    if let Some(new_coords) = self.move_coords(Direction::Right, map) {
                        result.push(new_coords);
                    }
                }
                Direction::Left | Direction::Right => {
                    if let Some(new_coords) = self.move_coords(Direction::Up, map) {
                        result.push(new_coords);
                    }
                    if let Some(new_coords) = self.move_coords(Direction::Down, map) {
                        result.push(new_coords);
                    }
                }
            }
        }
        result
    }
    // Creates a position in that direction
    fn move_coords(&self, direction: Direction, map: &[Vec<usize>]) -> Option<Position> {
        let max = self.max;
        let coords = self.coords;
        let result: Option<(usize, usize)> = match direction {
            Direction::Up => {
                if coords.0 > 0 {
                    Some((coords.0 - 1, coords.1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if coords.0 + 1 < max.0 {
                    Some((coords.0 + 1, coords.1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if coords.1 > 0 {
                    Some((coords.0, coords.1 - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if coords.1 + 1 < max.1 {
                    Some((coords.0, coords.1 + 1))
                } else {
                    None
                }
            }
        };
        if let Some(new_coords) = result {
            return Some(Position {
                coords: new_coords,
                heat_cost: self.heat_cost + map[new_coords.0][new_coords.1],
                direction,
                amount_in_direction: if direction == self.direction {
                    self.amount_in_direction + 1
                } else {
                    1
                },
                max,
            });
        }
        None
    }
    fn finished(&self, max: (usize, usize)) -> bool {
        self.coords.0 + 1 == max.0 && self.coords.1 + 1 == max.1
    }
    fn important_values(&self) -> ((usize, usize), Direction, usize) {
        (self.coords, self.direction, self.amount_in_direction)
    }
}
// Note that this implementation was borrowed from https://nickymeuleman.netlify.app/garden/aoc2023-day17
fn calc(data: &str) -> (usize, usize) {
    let map: Vec<Vec<usize>> = data
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let max = (map.len(), map[0].len());
    let mut seen: HashSet<((usize, usize), Direction, usize)> = HashSet::new();
    let mut positions: BinaryHeap<Position> = BinaryHeap::new();
    let start = Position {
        coords: (0, 0),
        heat_cost: 0,
        direction: Direction::Right,
        amount_in_direction: 0,
        max: (map.len(), map[0].len()),
    };
    seen.insert(start.important_values());
    positions.push(start);
    let mut part1 = 0;
    'part1: while let Some(position) = positions.pop() {
        let neighbors = position.neighbors(0, 3, &map);
        for neighbor in neighbors {
            if neighbor.finished(max) {
                part1 = neighbor.heat_cost;
                break 'part1;
            }
            if seen.insert(neighbor.important_values()) {
                positions.push(neighbor);
            }
        }
    }
    let mut seen: HashSet<((usize, usize), Direction, usize)> = HashSet::new();
    let mut positions: BinaryHeap<Position> = BinaryHeap::new();
    let start = Position {
        coords: (0, 0),
        heat_cost: 0,
        direction: Direction::Right,
        amount_in_direction: 0,
        max: (map.len(), map[0].len()),
    };
    seen.insert(start.important_values());
    positions.push(start);
    let mut part2 = 0;
    'part2: while let Some(position) = positions.pop() {
        let neighbors = position.neighbors(4, 10, &map);
        for neighbor in neighbors {
            if neighbor.finished(max) {
                part2 = neighbor.heat_cost;
                break 'part2;
            }
            if seen.insert(neighbor.important_values()) {
                positions.push(neighbor);
            }
        }
    }
    (part1, part2)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 17 part 1 is {}.", part1);
    println!("Answer for day 17 part 2 is {}.", part2);
}
