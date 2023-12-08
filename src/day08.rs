use std::collections::HashMap;

// Used to get the data
const DATA: &str = include_str!("../data/final/day08.txt");
#[test]
fn test_part1() {
    const DATA_PARTA: &str = include_str!("../data/test/day08a.txt");
    assert!(part1(DATA_PARTA) == 2);
    const DATA_PARTB: &str = include_str!("../data/test/day08b.txt");
    assert!(part1(DATA_PARTB) == 6);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day08c.txt");
    assert!(part2(DATA_PART2) == 6);
}
#[test]
fn test_lcm() {
    assert!(lcm(vec![2, 3]) == 6);
    assert!(lcm(vec![2, 10001, 4]) == 40004);
}
#[test]
fn test_gen_primes() {
    assert!(gen_primes(11) == vec![2, 3, 5, 7, 11]);
}
fn gen_map(data: &str) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in data.lines() {
        let (key, value) = line.split_once(" = ").unwrap();
        let value = value.replace(['(', ')'], "");
        let (left, right) = value.split_once(", ").unwrap();
        map.insert(key.to_owned(), (left.to_owned(), right.to_owned()));
    }
    map
}
fn part1(data: &str) -> usize {
    let (instructions, map) = data.split_once("\n\n").unwrap();
    let map = gen_map(map);
    get_distance(&map, "AAA", instructions)
}
fn get_distance(
    map: &HashMap<String, (String, String)>,
    location: &str,
    instructions: &str,
) -> usize {
    let mut location = location;
    let mut count = 0;
    'a: loop {
        for instruction in instructions.chars() {
            count += 1;
            if instruction == 'L' {
                location = &map.get(location).unwrap().0;
            } else {
                location = &map.get(location).unwrap().1;
            }
        }
        if location.chars().last().expect("empty string") == 'Z' {
            break 'a;
        }
    }
    count
}
fn lcm(a: Vec<usize>) -> usize {
    let mut prime_factorization: Vec<usize> = vec![];
    let primes = gen_primes(*a.iter().max().unwrap());
    for i in a {
        let mut i = i;
        let mut factors = vec![];
        for j in primes.iter() {
            while i % j == 0 {
                i /= j;
                factors.push(j);
            }
        }
        let mut prime_factorization2 = prime_factorization.clone();
        for j in factors.iter() {
            match prime_factorization2.iter().position(|&x| x == **j) {
                Some(idx) => {
                    prime_factorization2.remove(idx);
                }
                None => prime_factorization.push(**j),
            }
        }
    }
    let mut result = 1;
    for i in prime_factorization {
        result *= i
    }
    result
}
fn gen_primes(n: usize) -> Vec<usize> {
    let mut primes = vec![2];
    for i in 3..(n + 1) {
        let mut is_prime = true;
        for prime in primes.iter() {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            primes.push(i);
        }
    }
    primes
}
fn part2(data: &str) -> usize {
    let (instructions, map) = data.split_once("\n\n").unwrap();
    let map = gen_map(map);
    let mut count = vec![];
    for str in map.keys() {
        if str.chars().last().expect("empty string") == 'A' {
            count.push(get_distance(&map, str, instructions));
        }
    }
    lcm(count)
}
pub fn main() {
    println!("Answer for day 8 part 1 is {}.", part1(DATA));
    println!("Answer for day 8 part 2 is {}.", part2(DATA));
}
