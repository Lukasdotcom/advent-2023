// Used to get the data
const DATA: &str = include_str!("../data/final/day05.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day05.txt");
    assert!(part1(DATA_PART1) == 35);
}
#[test]
fn test_seeds_to_next_part2() {
    let test_1 = seeds_to_next_part2(vec![(55, 13), (79, 14)], vec![(52, 50, 48), (50, 98, 2)]);
    assert!(test_1.contains(&(57, 13)) && test_1.contains(&(81, 14)) && test_1.len() == 2);
    let test_2 = seeds_to_next_part2(
        vec![(81, 14), (57, 13)],
        vec![(39, 0, 15), (0, 15, 37), (37, 52, 2)],
    );
    assert!(test_2.contains(&(57, 13)) && test_2.contains(&(81, 14)) && test_2.len() == 2);
    let test_3 = seeds_to_next_part2(
        vec![(81, 14), (57, 13)],
        vec![(42, 0, 7), (57, 7, 4), (0, 11, 42), (49, 53, 8)],
    );
    println!("test_3: {:?}", test_3);
    assert!(
        test_3.contains(&(53, 4))
            && test_3.contains(&(61, 9))
            && test_3.contains(&(81, 14))
            && test_3.len() == 3
    );
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day05.txt");
    assert!(part2(DATA_PART2) == 46);
}
fn gen_map(map: &str) -> Vec<(usize, usize, usize)> {
    let mut map: Vec<(usize, usize, usize)> = map
        .split_once(":\n")
        .unwrap()
        .1
        .lines()
        .map(|x| {
            let split: Vec<usize> = x.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
            (split[0], split[1], split[2])
        })
        .collect();
    map.sort_by(|a, b| a.1.cmp(&b.1));
    map
}
fn seeds_to_next_part1(seeds: Vec<usize>, map: Vec<(usize, usize, usize)>) -> Vec<usize> {
    seeds
        .into_iter()
        .map(|x| {
            for i in &map {
                if i.1 <= x && i.1 + i.2 > x {
                    return i.0 + x - i.1;
                }
                if i.1 > x {
                    break;
                }
            }
            x
        })
        .collect()
}
fn part1(data: &str) -> usize {
    let mut data = data.split("\n\n");
    let mut seeds: Vec<usize> = data
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    for x in data {
        seeds = seeds_to_next_part1(seeds, gen_map(x));
    }
    seeds.into_iter().min().unwrap()
}
fn seeds_to_next_part2(
    seeds: Vec<(usize, usize)>,
    map: Vec<(usize, usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut seeds2 = vec![];
    'a: for mut x in seeds {
        for i in &map {
            // Checks if there is an overlap in the beggining
            if x.0 >= i.1 && x.0 <= i.1 + i.2 {
                // Checks if this overlaps all the way to the end
                if x.0 + x.1 <= i.1 + i.2 {
                    seeds2.push((x.0 + i.0 - i.1, x.1));
                    continue 'a;
                } else {
                    let match_len = i.1 + i.2 - x.0;
                    seeds2.push((x.0 + i.0 - i.1, i.1 + i.2 - x.0));
                    x = (x.0 + match_len, x.1 - match_len);
                }
            }
            // Checks if there is an overlap
            else if i.1 > x.0 && i.1 < x.0 + x.1 {
                // Finds the part that is not overlapping at the start
                seeds2.push((x.0, i.1 - x.0));
                // Checks if the overlap goes to the end
                if x.0 + x.1 <= i.1 + i.2 {
                    let match_len = x.0 + x.1 - i.1;
                    seeds2.push((i.0, match_len));
                    continue 'a;
                }
                // Overlaps the middle
                else {
                    let match_len = i.1 + i.2 - x.0;
                    seeds2.push((i.0, match_len));
                    x = (x.0, x.1 - match_len);
                }
            }
            // Past the end
            else if x.0 + x.1 <= i.1 + i.2 {
                seeds2.push(x);
                continue 'a;
            }
        }
        seeds2.push(x);
    }
    seeds2
}
fn part2(data: &str) -> usize {
    let mut data = data.split("\n\n");
    let seeds_start: Vec<usize> = data
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut seeds = vec![];
    for x in 0..seeds_start.len() / 2 {
        seeds.push((seeds_start[2 * x], seeds_start[2 * x + 1]));
    }
    for x in data {
        seeds = seeds_to_next_part2(seeds, gen_map(x));
    }
    seeds.into_iter().map(|x| x.0).min().unwrap()
}
pub fn main() {
    println!("Answer for day 4 part 1 is {}.", part1(DATA));
    println!("Answer for day 4 part 2 is {}.", part2(DATA));
}
