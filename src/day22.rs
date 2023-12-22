use std::collections::HashSet;

// Used to get the data
const DATA: &str = include_str!("../data/final/day22.txt");
#[test]
fn test_part1() {
    const DATA_PARTA: &str = include_str!("../data/test/day22.txt");
    assert!(calc(DATA_PARTA).0 == 5);
}
#[test]
fn test_part2() {
    const DATA_PART: &str = include_str!("../data/test/day22.txt");
    assert!(calc(DATA_PART).1 == 7);
}
#[derive(Debug)]
struct Brick {
    x: usize,
    y: usize,
    z: usize,
    x2: usize,
    y2: usize,
    z2: usize,
    supporting: HashSet<usize>, // What the brick is supporting
    supported: HashSet<usize>,  // What the brick is being supported by
}
type Map = Vec<Vec<Vec<Option<usize>>>>;
fn place_brick(map: &mut Map, brick: &Brick, brick_id: usize) {
    #[allow(clippy::needless_range_loop)]
    for x in brick.x..=brick.x2 {
        for y in brick.y..=brick.y2 {
            for z in brick.z..=brick.z2 {
                map[x][y][z] = Some(brick_id);
            }
        }
    }
}
// This returns true if the brick is supported
fn fall_brick(map: &mut Map, brick: &mut Brick, brick_id: usize) -> (Vec<usize>, bool) {
    let mut supporting = vec![];
    if brick.z == 1 {
        return (supporting, true);
    }
    if !brick.supported.is_empty() {
        return (supporting, true);
    }
    let new_z = brick.z - 1;
    let mut fall = true;
    #[allow(clippy::needless_range_loop)]
    for x in brick.x..=brick.x2 {
        for y in brick.y..=brick.y2 {
            if map[x][y][new_z].is_some() {
                supporting.push(map[x][y][new_z].unwrap());
                brick.supported.insert(map[x][y][new_z].unwrap());
                fall = false;
            }
        }
    }
    if fall {
        brick.z -= 1;
        brick.z2 -= 1;
        #[allow(clippy::needless_range_loop)]
        for x in brick.x..=brick.x2 {
            for y in brick.y..=brick.y2 {
                map[x][y][brick.z] = Some(brick_id);
                map[x][y][brick.z2 + 1] = None;
            }
        }
        return (supporting, false);
    }
    (supporting, true)
}
fn calc(data: &str) -> (usize, usize) {
    let mut max = (0, 0, 0);
    let mut bricks: Vec<Brick> = data
        .lines()
        .map(|x| {
            let mut coord = x
                .split('~')
                .map(|x| x.split(',').map(|x| x.parse::<usize>().unwrap()));
            let mut coord1 = coord.next().unwrap();
            let mut coord2 = coord.next().unwrap();
            let brick = Brick {
                x: coord1.next().unwrap(),
                y: coord1.next().unwrap(),
                z: coord1.next().unwrap(),
                x2: coord2.next().unwrap(),
                y2: coord2.next().unwrap(),
                z2: coord2.next().unwrap(),
                supporting: HashSet::new(),
                supported: HashSet::new(),
            };
            if brick.x2 > max.0 {
                max.0 = brick.x2
            }
            if brick.y2 > max.1 {
                max.1 = brick.y2
            }
            if brick.z2 > max.2 {
                max.2 = brick.z2
            }
            brick
        })
        .collect();
    let mut map: Map = vec![vec![vec![None; max.2 + 1]; max.1 + 1]; max.0 + 1];
    bricks.sort_by(|a, b| a.z.cmp(&b.z));
    #[allow(clippy::needless_range_loop)]
    for brick in 0..bricks.len() {
        place_brick(&mut map, &bricks[brick], brick);
    }
    for brick in 0..bricks.len() {
        loop {
            let (supporting, done) = fall_brick(&mut map, &mut bricks[brick], brick);
            if done {
                for x in supporting {
                    bricks[x].supporting.insert(brick);
                }
                break;
            }
        }
    }
    let mut unsafe_to_remove = HashSet::new();
    bricks.iter().for_each(|x| {
        if x.supported.len() == 1 {
            unsafe_to_remove.insert(*x.supported.iter().next().unwrap());
        }
    });
    let part1 = bricks.len() - unsafe_to_remove.len();
    let part2: usize = unsafe_to_remove
        .iter()
        .map(|x| {
            // Finds the number of bricks that would fall if this brick was removed
            let mut check_these: Vec<usize> = vec![*x];
            let mut falling: HashSet<usize> = HashSet::new();
            'test: while let Some(brick_id) = check_these.pop() {
                if falling.contains(&brick_id) {
                    continue 'test;
                }
                let brick = &bricks[brick_id];
                // Checks if the brick is supported still
                if brick_id != *x {
                    for y in &brick.supported {
                        if !falling.contains(y) {
                            continue 'test;
                        }
                    }
                }
                falling.insert(brick_id);
                // Adds all the bricks that could fall if this brick was removed to a check list
                for y in &brick.supporting {
                    if !falling.contains(y) {
                        check_these.push(*y);
                    }
                }
            }
            falling.len() - 1
        })
        .sum();
    (part1, part2)
}

pub fn main() {
    let (part1, part2) = calc(DATA);
    println!("Answer for day 22 part 1 is {}.", part1);
    println!("Answer for day 22 part 2 is {}.", part2);
    // assert!(part2 == 103010);
}
