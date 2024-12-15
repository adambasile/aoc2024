use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::ops::{Add, Mul};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn constrain(&self, width: i64, height: i64) -> Self {
        Self {
            x: self.x.rem_euclid(width),
            y: self.y.rem_euclid(height),
        }
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[derive(Clone)]
struct Robot {
    pos: Point,
    vel: Point,
}

impl Robot {
    fn position_after(&self, seconds: i64) -> Point {
        let delta = self.vel * seconds;
        self.pos + delta
    }
}

enum Cmp {
    SMALLER,
    EQUAL,
    LARGER,
}
fn cmp(a: i64, b: i64) -> Cmp {
    if a == b {
        Cmp::EQUAL
    } else if a < b {
        Cmp::SMALLER
    } else {
        Cmp::LARGER
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Quadrant {
    TOPLEFT,
    TOPRIGHT,
    BOTTOMLEFT,
    BOTTOMRIGHT,
}
fn calculate_safety_factor(final_positions: Vec<Point>, width: i64, height: i64) -> i64 {
    let mut quadrant_counts = HashMap::from([
        (Quadrant::TOPLEFT, 0),
        (Quadrant::TOPRIGHT, 0),
        (Quadrant::BOTTOMLEFT, 0),
        (Quadrant::BOTTOMRIGHT, 0),
    ]);

    for constrained_final in final_positions.iter().map(|p| p.constrain(width, height)) {
        let quadrant = match (
            cmp(constrained_final.x, width / 2),
            cmp(constrained_final.y, height / 2),
        ) {
            (Cmp::SMALLER, Cmp::SMALLER) => Some(Quadrant::TOPLEFT),
            (Cmp::LARGER, Cmp::SMALLER) => Some(Quadrant::TOPRIGHT),
            (Cmp::SMALLER, Cmp::LARGER) => Some(Quadrant::BOTTOMLEFT),
            (Cmp::LARGER, Cmp::LARGER) => Some(Quadrant::BOTTOMRIGHT),
            (_, _) => None,
        };
        if let Some(quadrant) = quadrant {
            *quadrant_counts.entry(quadrant).or_insert(0) += 1;
        }
    }

    quadrant_counts.values().fold(1, |acc, e| acc * e)
}

#[allow(dead_code)]
fn find_tree(robots: &Vec<Robot>, width: i64, height: i64) {
    for n in 0..20000 {
        let unique_robot_positions: HashSet<_> = robots
            .iter()
            .map(|r| r.position_after(n).constrain(width, height))
            .collect();

        let promising = (0..width)
            .map(|w| (0..height).map(move |h| (h, w)))
            .flatten()
            .any(|(h, w)| {
                (0..6).all(|i| unique_robot_positions.contains(&Point { x: h + i, y: w }))
            });
        if promising {
            save_robot_positions(&unique_robot_positions, n, width, height)
        }
    }
}

fn save_robot_positions(positions: &HashSet<Point>, n: i64, width: i64, height: i64) {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tmp")
        .join(format!("robot_{:03}.txt", n));

    let mut contents = String::new();
    for h in 0..height {
        for w in 0..width {
            contents.push(match positions.contains(&Point { x: w, y: h }) {
                true => '#',
                false => '.',
            })
        }
        contents.push('\n')
    }
    File::create(&filename)
        .unwrap()
        .write_all(contents.as_bytes())
        .unwrap();
}

pub(crate) fn day14(lines: Vec<String>) -> (i64, i64) {
    let robots = parse(&lines);

    let width = 101;
    let height = 103;

    let partone = calculate_safety_factor(
        robots
            .iter()
            .map(|robot| robot.position_after(100))
            .collect(),
        width,
        height,
    );
    // find_tree(&robots, width, height);
    (partone, 0)
}

fn parse(lines: &Vec<String>) -> Vec<Robot> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(&lines.join("\n"))
        .map(|c| {
            let [pos_x, pos_y, vel_x, vel_y] = c.extract().1;
            Robot {
                pos: Point {
                    x: pos_x.parse().unwrap(),
                    y: pos_y.parse().unwrap(),
                },
                vel: Point {
                    x: vel_x.parse().unwrap(),
                    y: vel_y.parse().unwrap(),
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_14_small() {
        let lines = read_testfile("day14test.txt");
        assert_eq!(day14(lines), (21, 0));
    }

    #[test]
    fn test_day_14() {
        let lines = read_testfile("day14.txt");
        assert_eq!(day14(lines), (221142636, 0));
    }
}
