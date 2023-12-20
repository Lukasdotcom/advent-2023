use std::collections::{HashMap, HashSet, VecDeque};

use crate::day08;

// Used to get the data
const DATA: &str = include_str!("../data/final/day20.txt");
#[test]
fn test_part1() {
    const DATA_PARTA: &str = include_str!("../data/test/day20a.txt");
    assert!(calc(DATA_PARTA, false).0 == 32000000);
    const DATA_PARTB: &str = include_str!("../data/test/day20b.txt");
    assert!(calc(DATA_PARTB, false).0 == 11687500);
}
// There were no tests for part2
// #[test]
// fn test_part2() {
//     const DATA_PART: &str = include_str!("../data/test/day20.txt");
//     assert!(calc(DATA_PART).1 == 167409079868000);
// }
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PulseType {
    High,
    Low,
    None,
}
struct Pulse<'a> {
    from: &'a str,
    location: &'a str,
    pulse: PulseType,
}
fn send_pulse(
    map: &mut Map,
    important_signals: &mut HashMap<&str, Option<usize>>,
    round: usize,
) -> (usize, usize, bool) {
    let mut pulses: VecDeque<Pulse> = VecDeque::new();
    pulses.push_back(Pulse {
        from: "button",
        location: "broadcaster",
        pulse: PulseType::Low,
    });
    let mut high_pulse = 0;
    let mut low_pulse = 0;
    let mut important_signal_found = false;
    while let Some(Pulse {
        from,
        location,
        pulse,
    }) = pulses.pop_front()
    {
        if pulse == PulseType::High {
            high_pulse += 1;
        } else {
            low_pulse += 1;
        }
        let place = map.get_mut(location);
        if let Some(place) = place {
            let (send, locations) = match place {
                Object::Broadcaster(locations) => (pulse, locations),
                Object::FlipFlop(locations, ref mut state) => {
                    if pulse == PulseType::High {
                        (PulseType::None, locations)
                    } else {
                        *state = !*state;
                        if state == &true {
                            (PulseType::High, locations)
                        } else {
                            (PulseType::Low, locations)
                        }
                    }
                }
                Object::Conjunction(locations, ref mut set) => {
                    if pulse == PulseType::Low {
                        set.insert(from);
                    } else {
                        set.remove(from);
                    }
                    if set.is_empty() {
                        (PulseType::Low, locations)
                    } else {
                        if let Some(important) = important_signals.get_mut(location) {
                            *important = Some(round);
                            important_signal_found = true;
                        }
                        (PulseType::High, locations)
                    }
                }
            };
            if send == PulseType::None {
                continue;
            }
            for connection in locations.iter() {
                pulses.push_back(Pulse {
                    from: location,
                    location: connection,
                    pulse: send,
                });
            }
        }
    }
    (high_pulse, low_pulse, important_signal_found)
}
#[derive(Debug)]
enum Object<'a> {
    Broadcaster(Vec<&'a str>),
    FlipFlop(Vec<&'a str>, bool),
    Conjunction(Vec<&'a str>, HashSet<&'a str>),
}
type Map<'a> = HashMap<&'a str, Object<'a>>;
fn calc(data: &str, part2: bool) -> (usize, usize) {
    let mut conjunctions = Vec::new();
    let mut parent_of_rx = "";
    let mut map: Map = HashMap::from_iter(data.lines().map(|x| {
        let (name, connections) = x.split_once(" -> ").unwrap();
        let name_part = &name[1..];
        let connections: Vec<&str> = connections.split(", ").collect();
        match &name[0..1] {
            "b" => (name, Object::Broadcaster(connections)),
            "%" => (name_part, Object::FlipFlop(connections, false)),
            "&" => {
                conjunctions.push(name_part);
                if connections.contains(&"rx") {
                    parent_of_rx = name_part;
                }
                (name_part, Object::Conjunction(connections, HashSet::new()))
            }
            _ => unreachable!(),
        }
    }));
    // These are the signals that all have to release a high pulse at the same time for part 2
    let mut important_signals: HashMap<&str, Option<usize>> = HashMap::new();
    for name in conjunctions {
        let count: HashSet<&str> = map
            .iter()
            .filter(|x| match x.1 {
                Object::Broadcaster(a) => a.contains(&name),
                Object::FlipFlop(a, _) => a.contains(&name),
                Object::Conjunction(a, _) => a.contains(&name),
            })
            .map(|x| *x.0)
            .collect();
        if let Object::Conjunction(_, ref mut set) = map.get_mut(name).unwrap() {
            if name == parent_of_rx {
                important_signals = HashMap::from_iter(count.iter().map(|x| (*x, None)));
            }
            set.extend(count);
        }
    }
    let mut high_pulse = 0;
    let mut low_pulse = 0;
    let range = if part2 { 1..usize::MAX } else { 1..1001 };
    let mut part2 = 0;
    for x in range {
        let new = send_pulse(&mut map, &mut important_signals, x);
        if x < 1001 {
            high_pulse += new.0;
            low_pulse += new.1;
        }
        if x > 1 && new.2 && important_signals.iter().all(|x| x.1.is_some()) {
            part2 = day08::lcm(important_signals.values().map(|x| x.unwrap()).collect());
            break;
        }
    }
    (high_pulse * low_pulse, part2)
}
pub fn main() {
    let (part1, part2) = calc(DATA, false);
    println!("Answer for day 20 part 1 is {}.", part1);
    println!("Answer for day 20 part 2 is {}.", part2);
}
