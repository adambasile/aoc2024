use std::collections::HashMap;

pub(crate) fn day01(lines: Vec<String>) -> (i32, i32) {
    let parsed_lines: Vec<(i32, i32)> = lines
        .iter()
        .map(|l| {
            let mut parsed = l.split_whitespace().map(|w| w.parse::<i32>().unwrap());
            (parsed.next().unwrap(), parsed.next().unwrap())
        })
        .collect();
    let left: Vec<i32> = parsed_lines.iter().map(|(val, _)| val.clone()).collect();
    let right: Vec<i32> = parsed_lines.iter().map(|(_, val)| val.clone()).collect();

    let partone = sort(&left)
        .iter()
        .zip(sort(&right).iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    let mut right_frequencies: HashMap<&i32, i32> = HashMap::new();
    for i in right.iter() {
        *right_frequencies.entry(i).or_insert(0) += 1;
    }

    let parttwo: i32 = left
        .iter()
        .map(|x| x * right_frequencies.get(x).unwrap_or(&0))
        .sum();

    (partone, parttwo)
}

fn sort(vec: &Vec<i32>) -> Vec<i32> {
    let mut sorted = vec.clone();
    sorted.sort();
    sorted
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_01_small() {
        let lines = read_testfile("day01test.txt");
        assert_eq!(day01(lines), (11, 31));
    }

    #[test]
    fn test_day_01() {
        let lines = read_testfile("day01.txt");
        assert_eq!(day01(lines), (1651298, 21306195));
    }
}
