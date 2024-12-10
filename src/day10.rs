use std::collections::{HashMap, HashSet};

type Topography = HashMap<(i32, i32), usize>;

fn neighbours(&(h, w): &(i32, i32)) -> [(i32, i32); 4] {
    [(h - 1, w), (h, w - 1), (h + 1, w), (h, w + 1)]
}

pub(crate) fn day10(lines: Vec<String>) -> (i64, i64) {
    let topography = parse(lines);

    let trailheads = topography
        .iter()
        .filter(|(_, &val)| val == 0)
        .map(|(&pos, _)| pos);

    let mut paths: Vec<_> = trailheads.map(|pos| vec![pos]).collect();
    for i in 1..=9 {
        let mut new_paths = Vec::new();
        for path in &paths {
            for neighbour in neighbours(path.last().unwrap()) {
                match topography.get(&neighbour) {
                    None => {}
                    Some(&val) => {
                        if val == i {
                            let mut new_path = path.clone();
                            new_path.push(neighbour);
                            new_paths.push(new_path);
                        }
                    }
                }
            }
        }
        paths = new_paths;
    }

    let partone = paths
        .iter()
        .map(|path| (path.first().unwrap(), path.last().unwrap()))
        .collect::<HashSet<_>>()
        .len() as i64;
    let parttwo = paths.len() as i64;
    (partone, parttwo)
}

fn parse(lines: Vec<String>) -> Topography {
    lines
        .iter()
        .enumerate()
        .map(|(h, line)| {
            line.chars()
                .enumerate()
                .map(move |(w, c)| ((w as i32, h as i32), c.to_digit(10).unwrap() as usize))
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_10_small() {
        let lines = read_testfile("day10test.txt");
        assert_eq!(day10(lines), (36, 81));
    }

    #[test]
    fn test_day_10() {
        let lines = read_testfile("day10.txt");
        assert_eq!(day10(lines), (760, 1764));
    }
}
