use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct AntennaPair {
    a: Point,
    b: Point,
}

impl AntennaPair {
    fn antinodes(&self) -> [Point; 2] {
        let a_to_b = self.b - self.a;
        [self.a - a_to_b, self.b + a_to_b]
    }

    fn antinodes_repeating(&self, n: i32) -> Vec<Point> {
        let a_to_b = self.b - self.a;
        (0..n)
            .map(|i| [self.a - (a_to_b * i), self.b + (a_to_b * i)])
            .flatten()
            .collect()
    }
}

pub(crate) fn day08(lines: Vec<String>) -> FunctionOutput {
    let antennas = parse(&lines);
    let x_limit = lines[0].chars().count() as i32;
    let y_limit = lines.len() as i32;
    let in_bounds = |p: &Point| p.x >= 0 && p.y >= 0 && p.x < x_limit && p.y < y_limit;

    let antenna_pairs: Vec<_> = antennas
        .iter()
        .map(|(_, antennas)| {
            antennas
                .iter()
                .map(|a| {
                    antennas
                        .iter()
                        .filter(move |&b| b != a)
                        .map(|b| AntennaPair {
                            a: a.clone(),
                            b: b.clone(),
                        })
                })
                .flatten()
        })
        .flatten()
        .collect();

    let antinodes_p1: HashSet<Point> = antenna_pairs
        .iter()
        .map(|ap| ap.antinodes())
        .flatten()
        .filter(in_bounds)
        .collect();

    let antinodes_p2: HashSet<Point> = antenna_pairs
        .iter()
        .map(|ap| ap.antinodes_repeating(max(x_limit, y_limit)))
        .flatten()
        .filter(in_bounds)
        .collect();

    let partone = antinodes_p1.len() as i64;
    let parttwo = antinodes_p2.len() as i64;
    IntPair(partone, parttwo)
}

fn parse(lines: &Vec<String>) -> HashMap<char, Vec<Point>> {
    let mut antennas = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                freq => {
                    antennas.entry(freq).or_insert(Vec::new()).push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
    }
    antennas
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_08_small() {
        let lines = read_testfile("day08test.txt");
        assert_eq!(day08(lines), IntPair(14, 34));
    }

    #[test]
    fn test_day_08() {
        let lines = read_testfile("day08.txt");
        assert_eq!(day08(lines), IntPair(376, 1352));
    }
}
