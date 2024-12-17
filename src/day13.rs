use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize_x: i64,
    prize_y: i64,
}

impl Machine {
    fn add_ten_trillion(&self) -> Self {
        let ten_trillion = 10000000000000;
        Self {
            button_a: self.button_a,
            button_b: self.button_b,
            prize_x: self.prize_x + ten_trillion,
            prize_y: self.prize_y + ten_trillion,
        }
    }
    fn optimise(&self) -> Option<(i64, i64)> {
        let matrix = Matrix2x2 {
            a: self.button_a.x,
            b: self.button_b.x,
            c: self.button_a.y,
            d: self.button_b.y,
        };

        let result = matrix.adjugate().multiply((self.prize_x, self.prize_y));
        let determinant = matrix.determinant();

        match (result.0 % determinant, result.1 % determinant) {
            (0, 0) => Some((result.0 / determinant, result.1 / determinant)),
            _ => None,
        }
    }

    fn cost(&self) -> i64 {
        match self.optimise() {
            Some((prize_x, prize_y)) => prize_x * 3 + prize_y,
            None => 0,
        }
    }
}

#[derive(Debug)]
struct Matrix2x2 {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl Matrix2x2 {
    fn multiply(&self, (a, b): (i64, i64)) -> (i64, i64) {
        (a * self.a + b * self.b, a * self.c + b * self.d)
    }

    fn adjugate(&self) -> Matrix2x2 {
        Matrix2x2 {
            a: self.d,
            b: -self.b,
            c: -self.c,
            d: self.a,
        }
    }

    fn determinant(&self) -> i64 {
        self.a * self.d - self.b * self.c
    }
}

pub(crate) fn day13(lines: Vec<String>) -> FunctionOutput {
    let machines = parse(lines);
    let partone = machines.iter().map(|m| m.cost()).sum();
    let parttwo = machines.iter().map(|m| m.add_ten_trillion().cost()).sum();
    IntPair(partone, parttwo)
}

fn parse(lines: Vec<String>) -> Vec<Machine> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let machines = re
        .captures_iter(lines.join("\n").as_str())
        .map(|c| {
            let [button_a_x, button_a_y, button_b_x, button_b_y, prize_x, prize_y] = c.extract().1;
            Machine {
                button_a: Button {
                    x: button_a_x.parse().unwrap(),
                    y: button_a_y.parse().unwrap(),
                },
                button_b: Button {
                    x: button_b_x.parse().unwrap(),
                    y: button_b_y.parse().unwrap(),
                },
                prize_x: prize_x.parse().unwrap(),
                prize_y: prize_y.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    machines
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_13_small() {
        let lines = read_testfile("day13test.txt");
        assert_eq!(day13(lines), IntPair(480, 875318608908));
    }

    #[test]
    fn test_day_13() {
        let lines = read_testfile("day13.txt");
        assert_eq!(day13(lines), IntPair(26005, 105620095782547));
    }
}
