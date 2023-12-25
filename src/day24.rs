// Used to get the data
const DATA: &str = include_str!("../data/final/day24.txt");
#[test]
fn test_part1() {
    const DATA_PARTA: &str = include_str!("../data/test/day24.txt");
    assert!(calc(DATA_PARTA, 7.0, 27.0).0 == 2);
}
// Didn't have time for part2
// #[test]
// fn test_part2() {
//     const DATA_PART: &str = include_str!("../data/test/day24.txt");
//     assert!(calc(DATA_PART, 7.0, 27.0).1 == 47);
// }
#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    xv: f64,
    yv: f64,
    zv: f64,
}
// Does a rref on a matrix
fn rref(matrix: Vec<Vec<f64>>) -> Option<Vec<f64>> {
    let mut matrix = matrix;
    for i in 0..matrix.len() {
        if matrix[i][i] == 0.0 && i < matrix.len() - 1 {
            matrix[i] = matrix[i + 1].clone();
        }
        if matrix[i][i] != 0.0 {
            let divisor = matrix[i][i];
            matrix[i][i] = 1.0;
            // Makes the row start with a 1
            if divisor != 1.0 {
                for j in i + 1..matrix[0].len() {
                    matrix[i][j] /= divisor;
                }
            }
        }
        // Makes that column 0 in all other rows
        for j in 0..matrix.len() {
            if j != i {
                let factor = matrix[j][i];
                matrix[j][i] = 0.0;
                for k in i + 1..matrix[0].len() {
                    matrix[j][k] -= matrix[i][k] * factor;
                }
            }
        }
        if matrix[i][i] == 0.0 {
            for k in i + 1..matrix[0].len() {
                matrix[i][k] = 0.0;
            }
        }
    }
    let result: Vec<f64> = matrix.iter().map(|x| *x.last().unwrap()).collect();
    let mut max = matrix[0].len() - 1;
    if result.len() - 1 < max {
        max = result.len() - 1;
    }
    if result[0..max].iter().all(|x| x <= &0.0) && (result[max] - 1.0).abs() <= 0.0 {
        return None;
    }
    Some(result)
}
fn check_collision(p1: &Point, p2: &Point, min: f64, max: f64) -> bool {
    let a = rref(vec![
        vec![p1.xv, -p2.xv, p2.x - p1.x],
        vec![p1.yv, -p2.yv, p2.y - p1.y],
    ]);
    if let Some(a) = a {
        if a.iter().any(|x| *x <= 0.0) {
            return false;
        }
        if (p1.x + p1.xv * a[0]) < min || (p1.x + p1.xv * a[0]) > max {
            return false;
        }
        if (p1.y + p1.yv * a[0]) < min || (p1.y + p1.yv * a[0]) > max {
            return false;
        }
        return true;
    }
    false
}
fn calc(data: &str, min: f64, max: f64) -> (usize, i64) {
    let mut part1 = 0;
    let points: Vec<Point> = data
        .lines()
        .map(|x| {
            let (pos, vel) = x.split_once('@').expect("");
            let mut pos = pos.split(", ").map(|x| x.trim().parse::<f64>().unwrap());
            let mut vel = vel.split(", ").map(|x| x.trim().parse::<f64>().unwrap());
            Point {
                x: pos.next().unwrap(),
                y: pos.next().unwrap(),
                z: pos.next().unwrap(),
                xv: vel.next().unwrap(),
                yv: vel.next().unwrap(),
                zv: vel.next().unwrap(),
            }
        })
        .collect();
    for x in 0..points.len() {
        let point1 = &points[x];
        #[allow(clippy::needless_range_loop)]
        for x2 in x + 1..points.len() {
            let point2 = &points[x2];
            if check_collision(point1, point2, min, max) {
                part1 += 1;
            }
        }
    }
    (part1, 0)
}

pub fn main() {
    let (part1, _) = calc(DATA, 200000000000000.0, 400000000000000.0);
    println!("Answer for day 24 part 1 is {}.", part1);
    // println!("Answer for day 24 part 2 is {}.", part2);
}
