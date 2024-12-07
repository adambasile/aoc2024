#[derive(Debug)]
struct Equation {
    lhs: i64,
    rhs: Vec<i64>,
}

impl From<&String> for Equation {
    fn from(s: &String) -> Self {
        let (lhs, rhs) = s.split_once(": ").unwrap();
        Self {
            lhs: lhs.parse().unwrap(),
            rhs: rhs.split_whitespace().map(|s| s.parse().unwrap()).collect(),
        }
    }
}

#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

fn combinations(n: usize) -> Vec<Vec<Operator>> {
    let mut out = vec![Vec::new()];
    for _ in 0..n {
        let mut new_out = Vec::new();
        for op in [Operator::Add, Operator::Multiply] {
            for old in out.iter() {
                let mut new = old.clone();
                new.push(op);
                new_out.push(new);
            }
        }
        out = new_out;
    }
    out
}

impl Equation {
    fn possible(&self) -> bool {
        let combinations = combinations(self.rhs.len() - 1);
        combinations.iter().any(|combination| {
            [Operator::Add]
                .iter()
                .chain(combination)
                .zip(self.rhs.iter())
                .fold(0, |acc, (op, val)| match op {
                    Operator::Add => acc + val,
                    Operator::Multiply => acc * val,
                })
                == self.lhs
        })
    }
}

pub(crate) fn day07(lines: Vec<String>) -> (i64, i64) {
    let p1_equations: Vec<Equation> = lines.iter().map(|s| s.into()).collect();
    let partone = (&p1_equations)
        .iter()
        .filter(|e| e.possible())
        .map(|e| e.lhs)
        .sum();
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_07_small() {
        let lines = read_testfile("day07test.txt");
        assert_eq!(day07(lines), (3749, 0));
    }

    #[test]
    fn test_day_07() {
        let lines = read_testfile("day07.txt");
        assert_eq!(day07(lines), (1298103531759, 0));
    }
}
