use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

// Used to get the data
const DATA: &str = include_str!("../data/final/day07.txt");
#[test]
fn test_part1() {
    const DATA_PART1: &str = include_str!("../data/test/day07.txt");
    assert!(part1(DATA_PART1) == 6440);
}
#[test]
fn test_part2() {
    const DATA_PART2: &str = include_str!("../data/test/day07.txt");
    assert!(part2(DATA_PART2) == 5905);
}
#[test]
fn test_get_level() {
    assert!(get_level(&['5', '6', '6', '6']) == Level::FourOfAKind);
    assert!(get_level(&['4', '5', '6', '3']) == Level::OnePair);
    assert!(get_level(&['6']) == Level::FiveOfAKind);
    assert!(get_level(&['K', 'K', 'A']) == Level::FourOfAKind);
    assert!(get_level(&['6', '5', '6', '9']) == Level::ThreeOfAKind);
    assert!(get_level(&[]) == Level::FiveOfAKind);
    assert!(get_level(&['3', 'A', '7', '7']) == Level::ThreeOfAKind);
    assert!(get_level(&['2', '2', '7', '7']) == Level::FullHouse);
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Level {
    High = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}
#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    level: Level,
}
fn get_level(cards: &[char]) -> Level {
    let jacks = 5 - cards.len();
    let unique_cards: HashSet<&char> = cards.iter().collect();
    if unique_cards.is_empty() {
        return Level::FiveOfAKind;
    }
    // Check for five of a kind
    if unique_cards.len() == 1 {
        return Level::FiveOfAKind;
    }
    // Check for four of a kind or full house
    if unique_cards.len() == 2 {
        let unique_cards: Vec<&char> = unique_cards.into_iter().collect();
        if cards.iter().filter(|x| *x == unique_cards[0]).count() > 1
            && cards.iter().filter(|x| *x == unique_cards[1]).count() > 1
        {
            return Level::FullHouse;
        } else {
            return Level::FourOfAKind;
        }
    }
    // Checks for a two pair
    if unique_cards.len() == 3 {
        for x in unique_cards {
            if cards.iter().filter(|y| *y == x).count() + jacks == 3 {
                return Level::ThreeOfAKind;
            }
        }
        return Level::TwoPair;
    }
    // Checks for a pair
    if unique_cards.len() == 4 {
        return Level::OnePair;
    }
    Level::High
}
fn part1(data: &str) -> usize {
    let char_to_int: HashMap<char, usize> = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    let mut hands: Vec<Hand> = data
        .lines()
        .map(|x| {
            let cards: Vec<char> = x.split_once(' ').unwrap().0.chars().collect();
            let bid = x.split_once(' ').unwrap().1.parse::<usize>().unwrap();
            Hand {
                bid,
                level: get_level(&cards),
                cards,
            }
        })
        .collect();
    hands.sort_by(|x, y| {
        if x.level == y.level {
            for i in 0..x.cards.len() {
                if x.cards[i] != y.cards[i] {
                    return char_to_int[&x.cards[i]].cmp(&char_to_int[&y.cards[i]]);
                }
            }
            Ordering::Equal
        } else {
            (x.level as usize).cmp(&(y.level as usize))
        }
    });
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (i + 1);
    }
    total
}
fn part2(data: &str) -> usize {
    let char_to_int: HashMap<char, usize> = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    let mut hands: Vec<Hand> = data
        .lines()
        .map(|x| {
            let cards: Vec<char> = x.split_once(' ').unwrap().0.chars().collect();
            let filtered: Vec<char> = cards.clone().into_iter().filter(|x| *x != 'J').collect();
            let bid = x.split_once(' ').unwrap().1.parse::<usize>().unwrap();
            Hand {
                bid,
                level: get_level(&filtered),
                cards,
            }
        })
        .collect();
    hands.sort_by(|x, y| {
        if x.level == y.level {
            for i in 0..x.cards.len() {
                if x.cards[i] != y.cards[i] {
                    return char_to_int[&x.cards[i]].cmp(&char_to_int[&y.cards[i]]);
                }
            }
            Ordering::Equal
        } else {
            (x.level as usize).cmp(&(y.level as usize))
        }
    });
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (i + 1);
    }
    total
}
pub fn main() {
    println!("Answer for day 7 part 1 is {}.", part1(DATA));
    println!("Answer for day 7 part 2 is {}.", part2(DATA));
}
