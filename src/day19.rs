use std::collections::HashMap;

// Used to get the data
const DATA: &str = include_str!("../data/final/day19.txt");
#[test]
fn test_part1() {
    const DATA_PART: &str = include_str!("../data/test/day19.txt");
    assert!(calc(DATA_PART).0 == 19114);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day19.txt");
    assert!(calc(DATA_PART).1 == 167409079868000);
}
#[derive(Debug)]
struct Part<'a> {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
    location: &'a str,
}
#[derive(Debug)]
enum Ratings {
    X,
    M,
    A,
    S,
}
#[derive(Debug)]
enum Rule {
    Greater(usize, Ratings, String),
    Lesser(usize, Ratings, String),
    Always(String),
}
#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
}
impl Rules {
    fn result<'a>(&'a self, part: Part<'a>) -> Part {
        let mut part: Part<'a> = part;
        for rule in &self.rules {
            match rule {
                Rule::Always(x) => {
                    part.location = x;
                    return part;
                }
                Rule::Greater(num, rating, goal) => {
                    let part_num = match rating {
                        Ratings::X => part.x,
                        Ratings::M => part.m,
                        Ratings::A => part.a,
                        Ratings::S => part.s,
                    };
                    if part_num > *num {
                        part.location = goal;
                        return part;
                    }
                }
                Rule::Lesser(num, rating, goal) => {
                    let part_num = match rating {
                        Ratings::X => part.x,
                        Ratings::M => part.m,
                        Ratings::A => part.a,
                        Ratings::S => part.s,
                    };
                    if part_num < *num {
                        part.location = goal;
                        return part;
                    }
                }
            }
        }
        panic!("Failed to match a rule for part");
    }
    fn new(data: &str) -> Self {
        let mut data = data.split(',');
        let mut rules = vec![];
        let last = Rule::Always(data.next_back().unwrap().to_owned());
        for x in data {
            let rating = match &x[0..1] {
                "x" => Ratings::X,
                "m" => Ratings::M,
                "a" => Ratings::A,
                "s" => Ratings::S,
                _ => panic!("Invalid rating"),
            };
            let (num, goal) = x[2..].split_once(':').expect("Invalid rule");
            let num = num.parse().unwrap();
            let goal = goal.to_owned();
            rules.push(match &x[1..2] {
                ">" => Rule::Greater(num, rating, goal),
                "<" => Rule::Lesser(num, rating, goal),
                _ => panic!("Invalid Operator"),
            })
        }
        rules.push(last);
        let rules: Self = Self { rules };
        rules
    }
}
#[derive(Debug)]
struct CompressedRule {
    min: MinOrMax,
    max: MinOrMax,
    accepted: bool,
}
#[derive(Clone, Debug)]
struct MinOrMax {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
fn compress(
    rules: &HashMap<&str, Rules>,
    min: MinOrMax,
    max: MinOrMax,
    location: &str,
) -> Vec<CompressedRule> {
    let mut result = vec![];
    if location == "A" {
        return vec![CompressedRule {
            min,
            max,
            accepted: true,
        }];
    }
    if location == "R" {
        return vec![CompressedRule {
            min,
            max,
            accepted: false,
        }];
    }
    let mut min = min;
    let mut max = max;
    for rule in &rules[location].rules {
        if min.x > max.x && min.m > max.m && min.a > max.a && min.s > max.s {
            break;
        }
        let new: Option<Vec<CompressedRule>> = match rule {
            Rule::Always(x) => {
                let new = compress(rules, min.clone(), max.clone(), x);
                max = MinOrMax {
                    x: 0,
                    m: 0,
                    a: 0,
                    s: 0,
                };
                Some(new)
            }
            Rule::Greater(num, rating, goal) => match *rating {
                Ratings::X => {
                    if max.x > *num && max.x > min.x {
                        let temp_min = min.x;
                        min.x = *num + 1;
                        let new = compress(rules, min.clone(), max.clone(), goal);
                        max.x = *num;
                        min.x = temp_min;
                        Some(new)
                    } else {
                        None
                    }
                }
                Ratings::M => {
                    if max.m > *num && max.m > min.m {
                        let temp_min = min.m;
                        min.m = *num + 1;
                        let new = compress(rules, min.clone(), max.clone(), goal);
                        max.m = *num;
                        min.m = temp_min;
                        Some(new)
                    } else {
                        None
                    }
                }
                Ratings::A => {
                    if max.a > *num && max.a > min.a {
                        let temp_min = min.a;
                        min.a = *num + 1;
                        let new = compress(rules, min.clone(), max.clone(), goal);
                        max.a = *num;
                        min.a = temp_min;
                        Some(new)
                    } else {
                        None
                    }
                }
                Ratings::S => {
                    if max.s > *num && max.s > min.s {
                        let temp_min = min.s;
                        min.s = *num + 1;
                        let new = compress(rules, min.clone(), max.clone(), goal);
                        max.s = *num;
                        min.s = temp_min;
                        Some(new)
                    } else {
                        None
                    }
                }
            },
            Rule::Lesser(num, rating, goal) => match *rating {
                Ratings::X => {
                    if min.x < *num && min.x < max.x {
                        let temp_max = max.x;
                        max.x = *num - 1;
                        let new = compress(rules, min.clone(), max.clone(), goal);
                        min.x = *num;
                        max.x = temp_max;
                        Some(new)
                    } else {
                        None
                    }
                }
                Ratings::M => {
                    if min.m < *num && min.m < max.m {
                        let temp_max = max.m;
                        max.m = *num - 1;
                        let new = compress(rules, min.clone(), max.clone(), goal);
                        min.m = *num;
                        max.m = temp_max;
                        Some(new)
                    } else {
                        None
                    }
                }
                Ratings::A => {
                    if min.a < *num && min.a < max.a {
                        let temp_max = max.a;
                        max.a = *num - 1;
                        let new = compress(rules, min.clone(), max.clone(), goal);
                        min.a = *num;
                        max.a = temp_max;
                        Some(new)
                    } else {
                        None
                    }
                }
                Ratings::S => {
                    if min.s < *num && min.s < max.s {
                        let temp_max = max.s;
                        max.s = *num - 1;
                        let new = compress(rules, min.clone(), max.clone(), goal);
                        min.s = *num;
                        max.s = temp_max;
                        Some(new)
                    } else {
                        None
                    }
                }
            },
        };
        if let Some(new) = new {
            result.extend(new);
        }
    }
    result
}
fn calc(data: &str) -> (usize, usize) {
    let (rules, parts) = data.split_once("\n\n").unwrap();
    let parts = parts
        .lines()
        .map(|x| {
            let mut ratings = x.split(',');
            let x = ratings.next().unwrap()[3..].parse().unwrap();
            let m = ratings.next().unwrap()[2..].parse().unwrap();
            let a = ratings.next().unwrap()[2..].parse().unwrap();
            let s = ratings.next().unwrap();
            let s = s[2..s.len() - 1].parse().unwrap();

            Part {
                x,
                m,
                a,
                s,
                location: "in",
            }
        })
        .collect::<Vec<_>>();
    let rules = rules.lines().map(|x| {
        let (name, rules): (&str, &str) = x.split_once('{').expect("Invalid rule");
        let rules = Rules::new(&rules[0..rules.len() - 1]);
        (name, rules)
    });
    let rules: HashMap<&str, Rules> = HashMap::from_iter(rules);
    let compress = compress(
        &rules,
        MinOrMax {
            x: 1,
            m: 1,
            a: 1,
            s: 1,
        },
        MinOrMax {
            x: 4000,
            m: 4000,
            a: 4000,
            s: 4000,
        },
        "in",
    );
    let mut part2 = 0;
    for x in compress {
        if x.accepted {
            part2 += (x.max.x - x.min.x + 1)
                * (x.max.m - x.min.m + 1)
                * (x.max.a - x.min.a + 1)
                * (x.max.s - x.min.s + 1);
        }
    }
    let mut part1 = 0;
    for part in parts {
        let mut part = part;
        loop {
            if part.location == "A" {
                part1 += part.x + part.m + part.a + part.s;
                break;
            }
            if part.location == "R" {
                break;
            }
            part = rules[part.location].result(part);
        }
    }
    (part1, part2)
}
pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 19 part 1 is {}.", part1);
    println!("Answer for day 19 part 2 is {}.", part2);
}
