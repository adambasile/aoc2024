use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;
use memoize::memoize;

pub(crate) fn day11(lines: Vec<String>) -> FunctionOutput {
    let stones: Vec<_> = lines[0]
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let partone = blink(&stones, 25) as i64;
    let parttwo = blink(&stones, 75) as i64;
    IntPair(partone, parttwo)
}

fn blink(stones: &Vec<u64>, n: u32) -> u64 {
    stones.iter().map(|&stone| blink_one(stone, n)).sum()
}

#[memoize]
fn blink_one(stone: u64, n: u32) -> u64 {
    if n == 0 {
        return 1;
    }
    let n = n - 1;

    if stone == 0 {
        blink_one(1, n)
    } else if (stone.ilog10() % 2) == 1 {
        let (first_half, second_half) = split_number(stone);
        blink_one(first_half, n) + blink_one(second_half, n)
    } else {
        blink_one(stone * 2024, n)
    }
}

fn split_number(stone: u64) -> (u64, u64) {
    let num_digits = stone.ilog10() + 1;
    let first_half = stone / 10_u64.pow(num_digits / 2);
    let second_half = stone % 10_u64.pow(num_digits / 2);
    (first_half, second_half)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day11test.txt");
        assert_eq!(day11(lines), IntPair(55312, 65601038650482));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day11.txt");
        assert_eq!(day11(lines), IntPair(203228, 240884656550923));
    }

    #[test]
    fn test_split_number() {
        assert_eq!(split_number(1234), (12, 34));
        assert_eq!(split_number(12345), (123, 45));
        assert_eq!(split_number(123456), (123, 456));
    }
}
