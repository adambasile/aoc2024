use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;

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
    Concatenate,
}

fn combinations(n: usize, with_concatenation: bool) -> Vec<Vec<Operator>> {
    let mut out = vec![Vec::new()];
    let operators = match with_concatenation {
        true => vec![Operator::Add, Operator::Multiply, Operator::Concatenate],
        false => vec![Operator::Add, Operator::Multiply],
    };

    for _ in 0..n {
        let mut new_out = Vec::new();
        for &op in operators.iter() {
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
    fn possible(&self, with_concatenation: bool) -> bool {
        let combinations = combinations(self.rhs.len() - 1, with_concatenation);
        combinations.iter().any(|combination| {
            [Operator::Add]
                .iter()
                .chain(combination)
                .zip(self.rhs.iter())
                .fold(0, |acc, (op, val)| match op {
                    Operator::Add => acc + val,
                    Operator::Multiply => acc * val,
                    Operator::Concatenate => (acc * (10_i64.pow(val.ilog10() + 1))) + val,
                })
                == self.lhs
        })
    }
}

pub(crate) fn day07(lines: Vec<String>) -> FunctionOutput {
    let equations: Vec<Equation> = lines.iter().map(|s| s.into()).collect();
    let partone = (&equations)
        .iter()
        .filter(|e| e.possible(false))
        .map(|e| e.lhs)
        .sum();
    let parttwo = (&equations)
        .iter()
        .filter(|e| e.possible(true))
        .map(|e| e.lhs)
        .sum();
    IntPair(partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_07_small() {
        let lines = read_testfile("day07test.txt");
        assert_eq!(day07(lines), IntPair(3749, 11387));
    }

    #[test]
    fn test_day_07() {
        let lines = read_testfile("day07.txt");
        assert_eq!(day07(lines), IntPair(1298103531759, 140575048428831));
    }
}
