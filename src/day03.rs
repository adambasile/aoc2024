use regex::Regex;

pub(crate) fn day03(lines: Vec<String>) -> (i32, i32) {
    let command = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let partone: i32 = lines
        .iter()
        .map(|l| command.captures_iter(l))
        .flatten()
        .map(|c| {
            (
                (&c[1]).parse::<i32>().unwrap(),
                (&c[2]).parse::<i32>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum();

    let conditional = Regex::new(r"don't\(\).+?(do\(\)|$)").unwrap();

    let command = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let parttwo: i32 = lines
        .iter()
        .map(|l| conditional.replace_all(l, "_"))
        .collect::<Vec<_>>()
        .iter()
        .map(|l| command.captures_iter(l))
        .flatten()
        .map(|c| {
            (
                (&c[1]).parse::<i32>().unwrap(),
                (&c[2]).parse::<i32>().unwrap(),
            )
        })
        .map(|(a, b)| a * b)
        .sum();
    (partone, parttwo)
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
        assert_eq!(day03(lines), (182780583, 97977612));
    }
}
