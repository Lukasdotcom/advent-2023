// Used to get the data
const DATA: &str = include_str!("../data/final/day23.txt");
#[test]
fn test_part1() {
    const DATA_PARTA: &str = include_str!("../data/test/day23.txt");
    assert!(calc(DATA_PARTA).0 == 94);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day23.txt");
    assert!(calc(DATA_PART).1 == 154);
}
#[derive(Debug)]
struct Neighbor {
    id: usize,
    dist: usize,
}
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    neighbors: Vec<Neighbor>,
    reverse_neighbors: Vec<Neighbor>,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
struct FindEnd {
    x: usize,
    y: usize,
    dist: usize,
    direction: Direction,
}
fn find_end(
    map: &[Vec<char>],
    direction: Direction,
    x: usize,
    y: usize,
    distance: usize,
) -> Option<FindEnd> {
    let char = map[x][y];
    if char == '#' {
        return None;
    }
    if x == map[0].len() - 1 && y == map.len() - 2 {
        return Some(FindEnd {
            x,
            y,
            dist: distance,
            direction,
        });
    }
    let mut valid_next_direction = vec![];
    if direction != Direction::Down && map[x - 1][y] != '#' {
        valid_next_direction.push(Direction::Up);
    }
    if direction != Direction::Up && map[x + 1][y] != '#' {
        valid_next_direction.push(Direction::Down);
    }
    if direction != Direction::Right && map[x][y - 1] != '#' {
        valid_next_direction.push(Direction::Left);
    }
    if direction != Direction::Left && map[x][y + 1] != '#' {
        valid_next_direction.push(Direction::Right);
    }
    match direction {
        Direction::Up => {
            if char == 'v' {
                return None;
            }
        }
        Direction::Down => {
            if char == '^' {
                return None;
            }
        }
        Direction::Left => {
            if char == '>' {
                return None;
            }
        }
        Direction::Right => {
            if char == '<' {
                return None;
            }
        }
    };
    if valid_next_direction.is_empty() {
        return None;
    }
    if valid_next_direction.len() == 1 {
        let new_coords = match valid_next_direction[0] {
            Direction::Up => (x - 1, y),
            Direction::Down => (x + 1, y),
            Direction::Left => (x, y - 1),
            Direction::Right => (x, y + 1),
        };
        return find_end(
            map,
            valid_next_direction[0],
            new_coords.0,
            new_coords.1,
            distance + 1,
        );
    }
    Some(FindEnd {
        x,
        y,
        dist: distance,
        direction,
    })
}
fn find_points(map: &[Vec<char>]) -> (usize, Vec<Point>) {
    let mut points = vec![Point {
        x: 0,
        y: 1,
        neighbors: vec![],
        reverse_neighbors: vec![],
    }];
    let mut end_point_id = 0;
    let mut new: Vec<(usize, Direction)> = vec![(0, Direction::Down)];
    while let Some(new_data) = new.pop() {
        let mut length = points.len() - 1;
        let direction = new_data.1;
        if points[new_data.0].x == map[0].len() - 1 && points[new_data.0].y == map.len() - 2 {
            end_point_id = new_data.0;
            continue;
        }
        if direction != Direction::Down {
            if let Some(end) = find_end(
                map,
                Direction::Up,
                points[new_data.0].x - 1,
                points[new_data.0].y,
                1,
            ) {
                let idx = points.iter().position(|x| x.x == end.x && x.y == end.y);
                if let Some(idx) = idx {
                    points[new_data.0].neighbors.push(Neighbor {
                        id: idx,
                        dist: end.dist,
                    });
                    points[idx].reverse_neighbors.push(Neighbor {
                        id: new_data.0,
                        dist: end.dist,
                    });
                } else {
                    points.push(Point {
                        x: end.x,
                        y: end.y,
                        neighbors: vec![],
                        reverse_neighbors: vec![],
                    });
                    length += 1;
                    new.push((length, end.direction));
                    points[new_data.0].neighbors.push(Neighbor {
                        id: length,
                        dist: end.dist,
                    });
                    points[length].reverse_neighbors.push(Neighbor {
                        id: new_data.0,
                        dist: end.dist,
                    });
                }
            }
        }
        if direction != Direction::Up {
            if let Some(end) = find_end(
                map,
                Direction::Down,
                points[new_data.0].x + 1,
                points[new_data.0].y,
                1,
            ) {
                let idx = points.iter().position(|x| x.x == end.x && x.y == end.y);
                if let Some(idx) = idx {
                    points[new_data.0].neighbors.push(Neighbor {
                        id: idx,
                        dist: end.dist,
                    });
                    points[idx].reverse_neighbors.push(Neighbor {
                        id: new_data.0,
                        dist: end.dist,
                    });
                } else {
                    points.push(Point {
                        x: end.x,
                        y: end.y,
                        neighbors: vec![],
                        reverse_neighbors: vec![],
                    });
                    length += 1;
                    new.push((length, end.direction));
                    points[new_data.0].neighbors.push(Neighbor {
                        id: length,
                        dist: end.dist,
                    });
                    points[length].reverse_neighbors.push(Neighbor {
                        id: new_data.0,
                        dist: end.dist,
                    });
                }
            }
        }
        if direction != Direction::Right {
            if let Some(end) = find_end(
                map,
                Direction::Left,
                points[new_data.0].x,
                points[new_data.0].y - 1,
                1,
            ) {
                let idx = points.iter().position(|x| x.x == end.x && x.y == end.y);
                if let Some(idx) = idx {
                    points[new_data.0].neighbors.push(Neighbor {
                        id: idx,
                        dist: end.dist,
                    });
                    points[idx].reverse_neighbors.push(Neighbor {
                        id: new_data.0,
                        dist: end.dist,
                    });
                } else {
                    points.push(Point {
                        x: end.x,
                        y: end.y,
                        neighbors: vec![],
                        reverse_neighbors: vec![],
                    });
                    length += 1;
                    new.push((length, end.direction));
                    points[new_data.0].neighbors.push(Neighbor {
                        id: length,
                        dist: end.dist,
                    });
                    points[length].reverse_neighbors.push(Neighbor {
                        id: new_data.0,
                        dist: end.dist,
                    });
                }
            }
        }
        if direction != Direction::Left {
            if let Some(end) = find_end(
                map,
                Direction::Right,
                points[new_data.0].x,
                points[new_data.0].y + 1,
                1,
            ) {
                let idx = points.iter().position(|x| x.x == end.x && x.y == end.y);
                if let Some(idx) = idx {
                    points[new_data.0].neighbors.push(Neighbor {
                        id: idx,
                        dist: end.dist,
                    });
                    points[idx].reverse_neighbors.push(Neighbor {
                        id: new_data.0,
                        dist: end.dist,
                    });
                } else {
                    points.push(Point {
                        x: end.x,
                        y: end.y,
                        neighbors: vec![],
                        reverse_neighbors: vec![],
                    });
                    length += 1;
                    new.push((length, end.direction));
                    points[new_data.0].neighbors.push(Neighbor {
                        id: length,
                        dist: end.dist,
                    });
                    points[length].reverse_neighbors.push(Neighbor {
                        id: new_data.0,
                        dist: end.dist,
                    });
                }
            }
        }
    }
    (end_point_id, points)
}
fn find_worst_path(points: &[Point], visited: Vec<usize>, end: usize, distance: usize) -> usize {
    let current = visited[visited.len() - 1];
    if current == end {
        return distance;
    }
    let current = &points[current];
    let mut max = 0;
    for neighbor in &current.neighbors {
        if !visited.contains(&neighbor.id) {
            let mut visited = visited.clone();
            visited.push(neighbor.id);
            max = max.max(find_worst_path(
                points,
                visited,
                end,
                distance + neighbor.dist,
            ))
        }
    }
    for neighbor in &current.reverse_neighbors {
        if !visited.contains(&neighbor.id) {
            let mut visited = visited.clone();
            visited.push(neighbor.id);
            max = max.max(find_worst_path(
                points,
                visited,
                end,
                distance + neighbor.dist,
            ))
        }
    }
    max
}
fn calc(data: &str) -> (usize, usize) {
    let map: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();
    let (end_point_id, points) = find_points(&map);
    let mut ratings = vec![0; points.len()];
    let mut search = vec![0];
    while let Some(new_data) = search.pop() {
        let point = &points[new_data];
        let rating = ratings[new_data];
        for neighbor in &point.neighbors {
            if ratings[neighbor.id] < rating + neighbor.dist {
                ratings[neighbor.id] = rating + neighbor.dist;
                search.push(neighbor.id);
            }
        }
    }
    let part1 = ratings[end_point_id];
    let part2 = find_worst_path(&points, vec![0], end_point_id, 0);
    (part1, part2)
}

pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 23 part 1 is {}.", part1);
    println!("Answer for day 23 part 2 is {}.", part2);
}
