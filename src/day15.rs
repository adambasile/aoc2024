use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use std::collections::{BTreeSet, HashMap};

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Obstacle {
    BOX,
    LEFTBOX,
    RIGHTBOX,
    WALL,
}

enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

fn carry_out(
    warehouse: &HashMap<Point, Obstacle>,
    robot: &Point,
    instructions: &Vec<Direction>,
) -> HashMap<Point, Obstacle> {
    let mut warehouse = warehouse.clone();
    let mut robot = robot.clone();

    'instructions: for instruction in instructions.iter() {
        let robot_next = robot.next(instruction);
        let mut to_push = HashMap::new();
        let mut to_consider_pushing = BTreeSet::from([robot_next]);
        while let Some(current) = to_consider_pushing.pop_first() {
            if to_push.contains_key(&current) {
                continue;
            }
            let (next, other_half) = match warehouse.get(&current) {
                Some(Obstacle::BOX) => {
                    let next = current.next(&instruction);
                    (next, None)
                }
                Some(Obstacle::LEFTBOX) => {
                    let next = current.next(&instruction);
                    let to_the_right = current.next(&Direction::RIGHT);
                    (next, Some(to_the_right))
                }
                Some(Obstacle::RIGHTBOX) => {
                    let next = current.next(&instruction);
                    let to_the_left = current.next(&Direction::LEFT);
                    (next, Some(to_the_left))
                }
                Some(Obstacle::WALL) => continue 'instructions,
                None => continue,
            };
            to_consider_pushing.insert(next);
            if let Some(other_half) = other_half {
                to_consider_pushing.insert(other_half);
            }

            to_push.insert(current, warehouse.get(&current).unwrap().clone());
        }
        for pos in to_push.keys() {
            warehouse.remove(&pos);
        }
        for (pos, &obstacle) in to_push.iter() {
            let point = pos.next(&instruction);
            warehouse.insert(point, obstacle);
        }

        robot = robot_next;
    }
    warehouse
}

pub(crate) fn day15(lines: Vec<String>) -> FunctionOutput {
    let (warehouse, wide_warehouse, robot, wide_robot, instructions) = parse(lines);

    let tidied_wide_warehouse = carry_out(&wide_warehouse, &wide_robot, &instructions);
    let tidied_warehouse = carry_out(&warehouse, &robot, &instructions);
    let partone = tidied_warehouse
        .iter()
        .filter(|(_, &obstacle)| obstacle == Obstacle::BOX)
        .map(|(Point { x, y }, _)| x + (100 * y))
        .sum();

    let parttwo = tidied_wide_warehouse
        .iter()
        .filter(|(_, &obstacle)| obstacle == Obstacle::LEFTBOX)
        .map(|(Point { x, y }, _)| x + (100 * y))
        .sum();
    IntPair(partone, parttwo)
}

#[allow(dead_code)]
fn print_out_warehouse(warehouse: &HashMap<Point, Obstacle>, robot: &Point) {
    let width = warehouse.keys().map(|p| p.x).max().unwrap();
    let height = warehouse.keys().map(|p| p.y).max().unwrap();

    let mut out: String = String::new();
    for h in 0..=height {
        for w in 0..=width {
            let point = Point { x: w, y: h };
            out.push(if robot == &point {
                '@'
            } else {
                match warehouse.get(&point) {
                    None => ' ',
                    Some(Obstacle::BOX) => 'O',
                    Some(Obstacle::WALL) => '#',
                    Some(Obstacle::LEFTBOX) => '[',
                    Some(Obstacle::RIGHTBOX) => ']',
                }
            })
        }
        out.push('\n');
    }
    println!("{}", out);
}

fn parse(
    lines: Vec<String>,
) -> (
    HashMap<Point, Obstacle>,
    HashMap<Point, Obstacle>,
    Point,
    Point,
    Vec<Direction>,
) {
    let mut warehouse = HashMap::new();
    let mut wide_warehouse = HashMap::new();
    let mut robot = None;
    let split_pos = lines.iter().position(|l| l == "").unwrap();
    for (j, line) in lines.iter().take(split_pos).enumerate() {
        for (i, c) in line.chars().enumerate() {
            let pos = Point {
                x: i as i64,
                y: j as i64,
            };
            let obstacle = match c {
                '#' => Obstacle::WALL,
                'O' => Obstacle::BOX,
                '@' => {
                    robot = Some(pos);
                    continue;
                }
                _ => continue,
            };
            warehouse.insert(pos, obstacle);
            let wide_pos_l = Point {
                x: pos.x * 2,
                y: pos.y,
            };
            let wide_pos_r = Point {
                x: wide_pos_l.x + 1,
                y: wide_pos_l.y,
            };
            match obstacle {
                Obstacle::BOX => {
                    wide_warehouse.insert(wide_pos_l, Obstacle::LEFTBOX);
                    wide_warehouse.insert(wide_pos_r, Obstacle::RIGHTBOX);
                }
                Obstacle::WALL => {
                    wide_warehouse.insert(wide_pos_l, Obstacle::WALL);
                    wide_warehouse.insert(wide_pos_r, Obstacle::WALL);
                }
                _ => panic!(),
            }
        }
    }
    let robot = robot.unwrap();
    let instructions: Vec<_> = lines
        .iter()
        .skip(split_pos)
        .map(|l| l.chars())
        .flatten()
        .map(|c| match c {
            '^' => Direction::UP,
            'v' => Direction::DOWN,
            '>' => Direction::RIGHT,
            '<' => Direction::LEFT,
            _ => panic!(),
        })
        .collect();

    let wide_robot = Point {
        x: robot.x * 2,
        y: robot.y,
    };
    (warehouse, wide_warehouse, robot, wide_robot, instructions)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_15_small1() {
        let lines = read_testfile("day15test1.txt");
        assert_eq!(day15(lines), IntPair(2028, 1751));
    }
    #[test]
    fn test_day_15_small2() {
        let lines = read_testfile("day15test2.txt");
        assert_eq!(day15(lines), IntPair(908, 618));
    }

    #[test]
    fn test_day_15() {
        let lines = read_testfile("day15.txt");
        assert_eq!(day15(lines), IntPair(1383666, 1412866));
    }
}
