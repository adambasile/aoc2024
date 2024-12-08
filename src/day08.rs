use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

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
}

pub(crate) fn day08(lines: Vec<String>) -> (i64, i64) {
    let antennas = parse(&lines);
    let x_limit = lines[0].chars().count() as i32;
    let y_limit = lines.len() as i32;

    let mut all_antinodes: HashSet<Point> = HashSet::new();

    for (_freq, antennas) in antennas.iter() {
        let antinodes = antennas
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
            .map(|ap| ap.antinodes())
            .flatten()
            .filter(|p| p.x >= 0 && p.y >= 0 && p.x < x_limit && p.y < y_limit)
            .collect::<Vec<_>>();
        all_antinodes.extend(antinodes);
    }

    let partone = all_antinodes.len() as i64;
    let parttwo = 0;
    (partone, parttwo)
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
        assert_eq!(day08(lines), (14, 0));
    }

    #[test]
    fn test_day_08() {
        let lines = read_testfile("day08.txt");
        assert_eq!(day08(lines), (376, 0));
    }
}
