// Used to get the data
const DATA: &str = include_str!("../data/final/day15.txt");
#[test]
fn test_part1() {
    const DATA_PART: &str = include_str!("../data/test/day15.txt");
    assert!(calc(DATA_PART).0 == 1320);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day15.txt");
    assert!(hash("qp") == 1);
    assert!(calc(DATA_PART).1 == 145);
}
fn hash(data: &str) -> usize {
    let mut total = 0;
    for char in data.chars() {
        total += char as usize;
        total *= 17;
        total %= 256;
    }
    total
}
fn calc(data: &str) -> (usize, usize) {
    let data = data.split(',').collect::<Vec<&str>>();
    let part1 = data.iter().map(|x| hash(x)).sum();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for x in data {
        if x.contains('=') {
            let (a, b) = x.split_once('=').unwrap();
            let hash = hash(a);
            let num = b.parse::<usize>().unwrap();
            if let Some(idx) = boxes[hash].iter().position(|x| x.0 == a) {
                boxes[hash][idx].1 = num;
            } else {
                boxes[hash].push((a, num));
            }
        } else {
            let code = &x[0..x.len() - 1];
            let hash = hash(code);
            if let Some(idx) = boxes[hash].iter().position(|x| x.0 == code) {
                boxes[hash].remove(idx);
            }
        }
    }
    let mut part2 = 0;
    let mut box_idx = 0;
    for x in boxes {
        let mut idx = 0;
        box_idx += 1;
        for num in x {
            idx += 1;
            part2 += box_idx * idx * num.1;
        }
    }
    (part1, part2)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 15 part 1 is {}.", part1);
    println!("Answer for day 15 part 2 is {}.", part2);
}
