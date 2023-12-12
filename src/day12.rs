// Used to get the data
const DATA: &str = include_str!("../data/final/day12.txt");
#[test]
fn test_part1() {
    const DATA_PART: &str = include_str!("../data/test/day12.txt");
    assert!(calc(DATA_PART).0 == 21);
}
#[test]
fn test_find_combinations() {
    assert!(find_combinations("###", &vec![3], &mut vec![vec![None; 30]; 30]) == 1);
    assert!(find_combinations("???.###", &vec![1, 1, 3], &mut vec![vec![None; 30]; 30]) == 1);
    assert!(
        find_combinations(
            ".??..??...?##.",
            &vec![1, 1, 3],
            &mut vec![vec![None; 30]; 30]
        ) == 4
    );
    assert!(find_combinations("###?", &vec![3], &mut vec![vec![None; 30]; 30]) == 1);
    assert!(
        find_combinations(
            "???.###????.###",
            &vec![1, 1, 3, 1, 1, 3],
            &mut vec![vec![None; 30]; 30]
        ) == 1
    );
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day12.txt");
    assert!(calc(DATA_PART).1 == 525152);
}
type Cache = Vec<Vec<Option<usize>>>;
fn find_combinations(string: &str, groups: &Vec<usize>, cache: &mut Cache) -> usize {
    let mut string = string.to_string();
    while !string.is_empty() && &string[0..1] == "." {
        string = string[1..].to_string();
    }
    let string = string;
    if string.is_empty() {
        if groups.is_empty() {
            return 1;
        }
        return 0;
    }
    if groups.is_empty() {
        if string.chars().filter(|x| x == &'#').count() == 0 {
            return 1;
        }
        return 0;
    }
    let mut combinations = 0;
    if let Some(x) = cache[string.len() - 1][groups.len() - 1] {
        return x;
    }
    let first_char = string.chars().next().unwrap();
    if first_char == '?' || first_char == '.' {
        combinations += find_combinations(&string[1..], groups, cache);
    }
    if !groups.is_empty() && (first_char == '?' || first_char == '#') {
        let chars_to_test = groups[0];
        if chars_to_test <= string.len() {
            let test_str = &string[0..chars_to_test];
            if test_str.chars().all(|x| x == '?' || x == '#') {
                let new_groups = groups[1..].to_vec();
                let new_string = &string[chars_to_test..];
                let next_char = new_string.chars().next();
                combinations += match next_char {
                    Some('?') | Some('.') => {
                        find_combinations(&new_string[1..], &new_groups, cache)
                    }
                    Some('#') => 0,
                    None => find_combinations(new_string, &new_groups, cache),
                    _ => panic!("Invalid Character"),
                };
            }
        }
    }
    cache[string.len() - 1][groups.len() - 1] = Some(combinations);
    combinations
}
#[derive(Debug)]
struct Data {
    groups: Vec<usize>,
    string: String,
}
fn calc(data: &str) -> (usize, usize) {
    let lines = data.lines().map(|x| {
        let split = x.split(' ').collect::<Vec<&str>>();
        Data {
            groups: split[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
            string: split[0].to_string(),
        }
    });
    let mut total = 0;
    let mut total2 = 0;
    for line in lines {
        let mut cache = vec![vec![None; line.groups.len()]; line.string.len()];
        let count = find_combinations(&line.string, &line.groups, &mut cache);
        total += count;
        let mut new_groups = vec![];
        let mut new_string = String::new();
        for x in 0..5 {
            for group in &line.groups {
                new_groups.push(*group);
            }
            if x != 0 {
                new_string += "?";
            }
            new_string += &line.string;
        }
        let mut cache = vec![vec![None; new_groups.len()]; new_string.len()];
        let count = find_combinations(&new_string, &new_groups, &mut cache);
        total2 += count;
    }
    (total, total2)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 12 part 1 is {}.", part1);
    println!("Answer for day 12 part 2 is {}.", part2);
}
