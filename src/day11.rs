// Used to get the data
const DATA: &str = include_str!("../data/final/day11.txt");
#[test]
fn test_part1() {
    const DATA_PART: &str = include_str!("../data/test/day11.txt");
    assert!(calc(DATA_PART).0 == 374);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day11.txt");
    let map: Vec<Vec<char>> = DATA_PART.lines().map(|x| x.chars().collect()).collect();
    assert!(get_total_distance(find_galaxies(&map, 10)) == 1030);
    assert!(get_total_distance(find_galaxies(&map, 100)) == 8410);
}
fn find_galaxies(map: &Vec<Vec<char>>, expansion: usize) -> Vec<(i64, i64)> {
    let expansion = expansion - 1;
    let mut empty_columns = vec![];
    for x in 0..map[0].len() {
        let mut duplicate = true;
        #[allow(clippy::needless_range_loop)]
        for y in 0..map.len() {
            if map[y][x] != '.' {
                duplicate = false;
            }
        }
        if duplicate {
            empty_columns.push(x);
        }
    }
    let mut locations = vec![];
    let empty = vec!['.'; map[0].len()];
    let mut expanded_y = 0;
    #[allow(clippy::needless_range_loop)]
    for y in 0..map.len() {
        if map[y] == empty {
            expanded_y += expansion;
        }
        let mut expanded_x = 0;
        for x in 0..map[y].len() {
            if empty_columns.binary_search(&x).is_ok() {
                expanded_x += expansion;
            }
            if map[y][x] == '#' {
                locations.push(((x + expanded_x) as i64, (y + expanded_y) as i64));
            }
        }
    }
    locations
}
fn get_total_distance(locations: Vec<(i64, i64)>) -> i64 {
    let mut total = 0;
    for gal1 in 0..locations.len() {
        for gal2 in gal1 + 1..locations.len() {
            let gal1 = locations[gal1];
            let gal2 = locations[gal2];
            total += (gal1.0 - gal2.0).abs() + (gal1.1 - gal2.1).abs();
        }
    }
    total
}
fn calc(data: &str) -> (i64, i64) {
    let map: Vec<Vec<char>> = data.lines().map(|x| x.chars().collect()).collect();
    (
        get_total_distance(find_galaxies(&map, 2)),
        get_total_distance(find_galaxies(&map, 1000000)),
    )
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 11 part 1 is {}.", part1);
    println!("Answer for day 11 part 2 is {}.", part2);
}
