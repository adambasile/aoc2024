use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
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
        let mut under_consideration = robot_next.clone();
        loop {
            match warehouse.get(&under_consideration) {
                Some(Obstacle::BOX) => under_consideration = under_consideration.next(&instruction),
                Some(Obstacle::WALL) => continue 'instructions,
                None => break,
            }
        }
        if let Some(old_box) = warehouse.remove(&robot_next) {
            warehouse.insert(under_consideration, old_box);
        }
        robot = robot_next;
    }
    warehouse
}

pub(crate) fn day15(lines: Vec<String>) -> (i64, i64) {
    let (warehouse, robot, instructions) = parse(lines);
    let messy_warehouse = carry_out(&warehouse, &robot, &instructions);
    let partone = messy_warehouse
        .iter()
        .filter(|(_, &obstacle)| obstacle == Obstacle::BOX)
        .map(|(Point { x, y }, _)| x + (100 * y))
        .sum();
    let parttwo = 0;
    (partone, parttwo)
}

fn parse(lines: Vec<String>) -> (HashMap<Point, Obstacle>, Point, Vec<Direction>) {
    let mut warehouse = HashMap::new();
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
    (warehouse, robot, instructions)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_15_small() {
        let lines = read_testfile("day15test.txt");
        assert_eq!(day15(lines), (2028, 0));
    }

    #[test]
    fn test_day_15() {
        let lines = read_testfile("day15.txt");
        assert_eq!(day15(lines), (1383666, 0));
    }
}
