// Used to get the data
const DATA: &str = include_str!("../data/final/day21.txt");
#[test]
fn test_part1() {
    const DATA_PARTA: &str = include_str!("../data/test/day21.txt");
    assert!(calc(DATA_PARTA, 6, 0).0 == 16);
}
// The actual input was easier then the sample for part2
// #[test]
// fn test_part2() {
//     const DATA_PART: &str = include_str!("../data/test/day21.txt");
//     assert!(calc(DATA_PART, 6, 6).1 == 16);
//     assert!(calc(DATA_PART, 6, 10).1 == 50);
//     assert!(calc(DATA_PART, 6, 50).1 == 1594);
//     assert!(calc(DATA_PART, 6, 100).1 == 6536);
//     assert!(calc(DATA_PART, 6, 500).1 == 167004);
// }
fn get_neighbors(x: usize, y: usize, distance: usize, map: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];
    if x > 0 && map[x - 1][y] == '.' {
        neighbors.push((x - 1, y));
    }
    if y > 0 && map[x][y - 1] == '.' {
        neighbors.push((x, y - 1));
    }
    if x < map.len() - 1 && map[x + 1][y] == '.' {
        neighbors.push((x + 1, y));
    }
    if y < map[0].len() - 1 && map[x][y + 1] == '.' {
        neighbors.push((x, y + 1));
    }
    if distance > 1 {
        let mut new_neighbors = vec![];
        for neighbor in neighbors {
            new_neighbors.extend(get_neighbors(neighbor.0, neighbor.1, distance - 1, map));
        }
        neighbors = new_neighbors;
    }
    neighbors
}
fn simulate_map(
    map: Vec<Vec<char>>,
    steps: usize,
    start_locations: Vec<(usize, usize)>,
) -> (Vec<Vec<char>>, Vec<(usize, usize)>) {
    let mut locations = start_locations;
    let mut map = map;
    for step in 0..steps {
        let mut new_locations = vec![];
        while let Some((x, y)) = locations.pop() {
            let neighbors = get_neighbors(x, y, 1, &map);
            for neighbor in neighbors {
                if map[neighbor.0][neighbor.1] == '.' {
                    if step % 2 == 1 {
                        map[neighbor.0][neighbor.1] = 'S';
                    } else {
                        map[neighbor.0][neighbor.1] = '0';
                    }
                    new_locations.push(neighbor);
                }
            }
        }
        if new_locations.is_empty() {
            break;
        }
        locations = new_locations;
    }
    (map, locations)
}
fn count_map(map: &[Vec<char>], even: bool) -> usize {
    map.iter()
        .map(|x| {
            x.iter()
                .filter(|y| **y == if even { 'S' } else { '0' })
                .count()
        })
        .sum()
}
fn calc(data: &str, steps: usize, steps2: usize) -> (usize, usize) {
    let mut locations = vec![];
    let mut start_map: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();
    // let mut start_map = vec![];
    for x in 0..start_map.len() {
        for y in 0..start_map[0].len() {
            if start_map[x][y] == 'S' {
                start_map[x][y] = '.';
                // start_map = map.clone();
                locations.push((x, y));
            }
        }
    }
    let (part1_map, locations) = simulate_map(start_map.clone(), steps, locations);
    let part1 = count_map(&part1_map, true);
    let map = simulate_map(part1_map, steps2, locations).0;
    let even_steps: usize = count_map(&map, true);
    let odd_steps: usize = count_map(&map, false);
    if steps2 == 0 {
        return (part1, 0);
    }
    let length = (steps2 - (start_map.len() / 2)) / start_map.len();
    let corners = [
        (0, start_map[0].len() / 2),
        (start_map.len() - 1, start_map[0].len() / 2),
        (start_map.len() / 2, 0),
        (start_map.len() / 2, start_map[0].len() - 1),
    ];
    // Calculates the size of all 4 corners
    let corners: usize = corners
        .iter()
        .map(|x| {
            count_map(
                &simulate_map(start_map.clone(), start_map.len(), vec![*x]).0,
                true,
            )
        })
        .sum();
    // Coordinates where edges start
    let edges = [
        (0, 0),
        (0, start_map[0].len() - 1),
        (start_map.len() - 1, 0),
        (start_map.len() - 1, start_map[0].len() - 1),
    ];
    // Counts the size of a small edge on each side
    let small_edges: usize = edges
        .iter()
        .map(|x| {
            count_map(
                &simulate_map(start_map.clone(), start_map.len() / 2 - 1, vec![*x]).0,
                (start_map.len() / 2 - 1) % 2 == 0,
            )
        })
        .sum();
    // Counts the size of a large edge on each side
    let large_edges: usize = edges
        .iter()
        .map(|x| {
            count_map(
                &simulate_map(
                    start_map.clone(),
                    start_map.len() / 2 - 1 + start_map.len(),
                    vec![*x],
                )
                .0,
                (start_map.len() / 2 - 1 + start_map.len()) % 2 == 0,
            )
        })
        .sum();
    // Calculates the amount of full even plots
    let full_even_plots = (length / 2 * 2).pow(2);
    // Calculates the amount of full odd plots
    let full_odd_plots = (length / 2 * 2 - 1).pow(2);
    let part2 = full_odd_plots * odd_steps
        + full_even_plots * even_steps
        + corners
        + large_edges * (length - 1)
        + small_edges * length;
    (part1, part2)
}

pub fn main() {
    let (part1, part2) = calc(DATA, 64, 26501365);
    println!("Answer for day 21 part 1 is {}.", part1);
    println!("Answer for day 21 part 2 is {}.", part2);
}
