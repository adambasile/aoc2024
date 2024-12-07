pub(crate) fn day02(lines: Vec<String>) -> (i64, i64) {
    let parsed: Vec<Vec<i64>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let partone = parsed.iter().filter(|&l| is_safe(l)).count() as i64;
    let parttwo = parsed.iter().filter(|&l| is_safe_with_dampener(l)).count() as i64;
    (partone, parttwo)
}

fn is_safe(level: &Vec<i64>) -> bool {
    let pairwise: Vec<_> = level.iter().zip(level.iter().skip(1)).collect();
    let all_decreasing = pairwise.iter().all(|(&l, &r)| l > r);
    let all_increasing = pairwise.iter().all(|(&l, &r)| l < r);
    let small_differences = pairwise
        .iter()
        .all(|(&l, &r)| (1..4).contains(&(l - r).abs()));
    (all_decreasing || all_increasing) && small_differences
}

fn is_safe_with_dampener(level: &Vec<i64>) -> bool {
    dampened_levels(level).iter().any(|l| is_safe(l))
}

fn dampened_levels(level: &Vec<i64>) -> Vec<Vec<i64>> {
    (0..level.len())
        .map(|i| [&level[..i], &level[(i + 1)..]].concat())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_02_small() {
        let lines = read_testfile("day02test.txt");
        assert_eq!(day02(lines), (2, 4));
    }

    #[test]
    fn test_day_02() {
        let lines = read_testfile("day02.txt");
        assert_eq!(day02(lines), (246, 318));
    }
}
