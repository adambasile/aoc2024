use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use std::cmp::PartialEq;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn next(&self, direction: Direction) -> Position {
        let Position { x, y } = *self;
        let next_position = match direction {
            Direction::North => Position { x: x - 1, y },
            Direction::East => Position { x, y: y + 1 },
            Direction::South => Position { x: x + 1, y },
            Direction::West => Position { x, y: y - 1 },
        };
        next_position
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Guard {
    pos: Position,
    direction: Direction,
}

impl Guard {
    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn next_position(&self) -> Position {
        self.pos.next(self.direction)
    }
}

#[derive(Debug)]
struct Area {
    width: i64,
    height: i64,
    guard: Guard,
    obstructions: HashSet<Position>,
    guard_visited: HashSet<Position>,
    guard_trail: HashSet<Guard>,
}

impl Area {
    fn clone(&self) -> Area {
        Area {
            width: self.width,
            height: self.height,
            guard: self.guard.clone(),
            obstructions: self.obstructions.clone(),
            guard_visited: self.guard_visited.clone(),
            guard_trail: self.guard_trail.clone(),
        }
    }
}

#[derive(Debug)]
enum GuardStatus {
    Moved,
    Left,
    Trapped,
}

enum PatrolStatus {
    Left,
    Trapped,
}
impl Area {
    fn move_guard(&mut self) -> GuardStatus {
        if self.guard_trail.contains(&self.guard) {
            return GuardStatus::Trapped;
        }
        let next_position = self.guard.next_position();
        if self.obstructions.contains(&next_position) {
            self.guard.rotate();
            return self.move_guard();
        }
        self.guard_trail.insert(self.guard.clone());
        self.guard_visited.insert(self.guard.pos);
        if !self.within_bounds(next_position) {
            return GuardStatus::Left;
        }
        self.guard.pos = next_position;
        GuardStatus::Moved
    }

    fn within_bounds(&self, next_position: Position) -> bool {
        next_position.x < self.width
            && next_position.y < self.height
            && next_position.x >= 0
            && next_position.y >= 0
    }

    fn patrol(&mut self) -> PatrolStatus {
        loop {
            match self.move_guard() {
                GuardStatus::Moved => {}
                GuardStatus::Left => return PatrolStatus::Left,
                GuardStatus::Trapped => return PatrolStatus::Trapped,
            }
        }
    }
}

pub(crate) fn day06(lines: Vec<String>) -> FunctionOutput {
    let mut guards = Vec::new();
    let mut obstructions = HashSet::new();

    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstructions.insert(Position {
                        x: x as i64,
                        y: y as i64,
                    });
                }
                '^' => guards.push(Guard {
                    pos: Position {
                        x: x as i64,
                        y: y as i64,
                    },
                    direction: Direction::North,
                }),
                _ => continue,
            }
        }
    }
    assert_eq!(guards.len(), 1);

    let mut area = Area {
        width: lines[0].len() as i64,
        height: lines.len() as i64,
        guard: guards[0],
        obstructions,
        guard_visited: Default::default(),
        guard_trail: Default::default(),
    };
    let orig_area = area.clone();

    area.patrol();

    let partone = area.guard_visited.len() as i64;

    let mut parttwo = 0;

    for prev_visited in area.guard_visited.iter() {
        if prev_visited == &orig_area.guard.pos {
            continue;
        }
        let mut new_area = orig_area.clone();
        new_area.obstructions.insert(*prev_visited);

        match new_area.patrol() {
            PatrolStatus::Left => {}
            PatrolStatus::Trapped => parttwo = parttwo + 1,
        }
    }

    IntPair(partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_06_small() {
        let lines = read_testfile("day06test.txt");
        assert_eq!(day06(lines), IntPair(41, 6));
    }

    #[test]
    fn test_day_06() {
        let lines = read_testfile("day06.txt");
        assert_eq!(day06(lines), IntPair(4515, 1309));
    }
}
