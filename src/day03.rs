use regex::Regex;

pub(crate) fn day03(lines: Vec<String>) -> (i32, i32) {
    let partone = multiply_some_numbers(&lines, false);
    let parttwo = multiply_some_numbers(&lines, true);
    (partone, parttwo)
}

fn multiply_some_numbers(lines: &Vec<String>, sanitise: bool) -> i32 {
    let command = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut combined = lines.join("_");
    if sanitise {
        let conditional = Regex::new(r"don't\(\).+?(do\(\)|$)").unwrap();
        combined = conditional.replace_all(&combined, "_").into();
    }
    command
        .captures_iter(&combined)
        .map(|c| {
            (
                (&c[1]).parse::<i32>().unwrap(),
                (&c[2]).parse::<i32>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_03_small() {
        let lines = read_testfile("day03test.txt");
        assert_eq!(day03(lines), (161, 48));
    }

    #[test]
    fn test_day_03() {
        let lines = read_testfile("day03.txt");
        assert_eq!(day03(lines), (182780583, 90772405));
    }
}