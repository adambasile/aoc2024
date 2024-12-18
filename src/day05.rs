use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;

#[derive(Debug)]
struct Rule {
    before: i64,
    after: i64,
}

impl Rule {
    fn satisfied(&self, input: &Vec<i64>) -> bool {
        match input.iter().position(|&x| x == self.after) {
            None => true,
            Some(after_position) => match input
                .iter()
                .skip(after_position)
                .position(|&x| x == self.before)
            {
                None => true,
                Some(_) => false,
            },
        }
    }

    fn satisfy(&self, input: &Vec<i64>) -> Vec<i64> {
        let mut out = input.clone();
        match out.iter().position(|&x| x == self.before) {
            None => {}
            Some(before_position) => match out.iter().position(|&x| x == self.after) {
                None => {}
                Some(after_position) => {
                    if before_position > after_position {
                        out.swap(before_position, after_position);
                    }
                }
            },
        }
        out
    }
}

pub(crate) fn day05(lines: Vec<String>) -> FunctionOutput {
    let split_point = lines.iter().position(|l| l == "").unwrap();
    let rules_input: Vec<_> = lines[..split_point].into();
    let updates_input: Vec<_> = lines[(split_point + 1)..].into();

    let rules: Vec<Rule> = rules_input
        .iter()
        .map(|line| {
            let (before, after) = match line.split_once("|") {
                None => {
                    println!("{:?}", line);
                    panic!()
                }
                Some(x) => x,
            };
            Rule {
                before: before.parse().unwrap(),
                after: after.parse().unwrap(),
            }
        })
        .collect();

    let updates: Vec<Vec<i64>> = updates_input
        .iter()
        .map(|line| line.split(",").map(|i| i.parse().unwrap()).collect())
        .collect();

    let partone = updates
        .iter()
        .filter(|pages| rules.iter().all(|rule| rule.satisfied(pages)))
        .map(|pages| pages.iter().nth(pages.len() / 2).unwrap())
        .sum();

    let parttwo = updates
        .iter()
        .filter(|pages| !rules.iter().all(|rule| rule.satisfied(pages)))
        .map(|pages| {
            let mut pages = pages.clone();
            while !rules.iter().all(|rule| rule.satisfied(&pages)) {
                for rule in rules.iter() {
                    pages = rule.satisfy(&pages)
                }
            }
            pages
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|pages| pages.iter().nth(pages.len() / 2).unwrap())
        .sum();
    IntPair(partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_05_small() {
        let lines = read_testfile("day05test.txt");
        assert_eq!(day05(lines), IntPair(143, 123));
    }

    #[test]
    fn test_day_05() {
        let lines = read_testfile("day05.txt");
        assert_eq!(day05(lines), IntPair(4185, 4480));
    }
}
