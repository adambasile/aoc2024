use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn next(&self, direction: &Direction) -> Point {
        let (dx, dy) = match direction {
            Direction::LEFT => (-1, 0),
            Direction::RIGHT => (1, 0),
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
        };
        Point {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT,
];

impl Direction {
    fn opposite(&self, other: &Self) -> bool {
        &self.reverse() == other
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
        }
    }

    fn to_int(&self) -> u8 {
        match self {
            Direction::UP => 0,
            Direction::RIGHT => 1,
            Direction::DOWN => 2,
            Direction::LEFT => 3,
        }
    }
}

impl PartialOrd<Self> for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Copy, Clone)]
struct Holder {
    cost: i64,
    point: Point,
    direction: Direction,
}

pub(crate) fn day16(lines: Vec<String>) -> FunctionOutput {
    let (maze_walls, start, end) = parse(lines);
    let (path_cost, num_on_path) = find_path_cost(maze_walls, start, end).unwrap();

    let partone = path_cost;
    let parttwo = num_on_path;
    IntPair(partone, parttwo)
}

fn find_path_cost(maze_walls: HashSet<Point>, start: Point, end: Point) -> Option<(i64, i64)> {
    let mut visited: HashMap<(Point, Direction), i64> = HashMap::new();
    let mut to_visit = BTreeSet::from([(
        Holder {
            cost: 0,
            point: start,
            direction: Direction::RIGHT,
        },
        None,
    )]);
    let mut min_cost = None;
    let mut breadcrumbs: HashMap<(Point, Direction), Vec<(Point, Direction)>> = HashMap::new();
    while let Some((current, previous)) = to_visit.pop_first() {
        if current.cost > min_cost.unwrap_or(i64::MAX) {
            break;
        }
        let visited_test = (current.point, current.direction);

        if let Some(previous) = previous {
            if let Some(&visited_cost) = visited.get(&visited_test) {
                if current.cost == visited_cost {
                    breadcrumbs.get_mut(&visited_test).unwrap().push(previous);
                }
                continue;
            }
            breadcrumbs.insert(visited_test, vec![previous]);
        }
        visited.insert(visited_test, current.cost);

        if current.point == end {
            min_cost = Some(current.cost);
            continue;
        }
        for &new_direction in DIRECTIONS.iter() {
            if current.direction.opposite(&new_direction) {
                continue;
            }
            let next_point = current.point.next(&new_direction);
            if maze_walls.contains(&next_point) {
                continue;
            }
            let added_cost = if current.direction != new_direction {
                1001
            } else {
                1
            };
            to_visit.insert((
                Holder {
                    point: next_point,
                    cost: current.cost + added_cost,
                    direction: new_direction,
                },
                Some((current.point, current.direction)),
            ));
        }
    }
    if let Some(min_cost) = min_cost {
        let visited_points = get_points_on_trail(breadcrumbs, end);
        Some((min_cost, visited_points.len() as i64))
    } else {
        None
    }
}

fn get_points_on_trail(
    breadcrumbs: HashMap<(Point, Direction), Vec<(Point, Direction)>>,
    end: Point,
) -> HashSet<Point> {
    let mut to_visit: Vec<_> = DIRECTIONS
        .iter()
        .map(|&d| (end, d))
        .filter(|x| breadcrumbs.contains_key(x))
        .collect();
    let mut visited = HashSet::new();
    while let Some(current) = to_visit.pop() {
        if !visited.insert(current) {
            continue;
        }
        if let Some(to_add) = breadcrumbs.get(&current) {
            to_visit.extend(to_add);
        }
    }
    let visited_points: HashSet<_> = visited.iter().map(|(p, _)| *p).collect();
    visited_points
}

#[allow(dead_code)]
fn print_out_maze(walls: &HashSet<Point>, marked: &HashSet<Point>) {
    let width = walls
        .iter()
        .chain(marked.iter())
        .map(|p| p.x)
        .max()
        .unwrap();
    let height = walls
        .iter()
        .chain(marked.iter())
        .map(|p| p.y)
        .max()
        .unwrap();

    let mut out: String = String::new();
    for h in 0..=height {
        for w in 0..=width {
            let point = Point { x: w, y: h };
            let wall_here = walls.contains(&point);
            let marked_here = marked.contains(&point);
            out.push(if wall_here && marked_here {
                'X'
            } else if wall_here {
                '#'
            } else if marked_here {
                'O'
            } else {
                ' '
            });
        }
        out.push('\n');
    }
    println!("{}", out);
}

fn parse(lines: Vec<String>) -> (HashSet<Point>, Point, Point) {
    let mut maze_walls = HashSet::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let pos = Point {
                x: x as i64,
                y: y as i64,
            };
            match ch {
                '#' => {
                    maze_walls.insert(pos);
                }
                'S' => {
                    start = Some(pos);
                }
                'E' => {
                    end = Some(pos);
                }
                _ => continue,
            };
        }
    }
    maze_walls.shrink_to_fit();
    let start = start.unwrap();
    let end = end.unwrap();
    (maze_walls, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_testfile;

    #[test]
    fn test_day_16_small1() {
        let lines = read_testfile("day16test1.txt");
        assert_eq!(day16(lines), IntPair(7036, 45));
    }
    #[test]
    fn test_day_16_small2() {
        let lines = read_testfile("day16test2.txt");
        assert_eq!(day16(lines), IntPair(11048, 64));
    }
    #[test]
    fn test_day_16() {
        let lines = read_testfile("day16.txt");
        assert_eq!(day16(lines), IntPair(65436, 489));
    }
}
