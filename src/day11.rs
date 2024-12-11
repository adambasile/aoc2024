use std::thread;

pub(crate) fn day11(lines: Vec<String>) -> (i64, i64) {
    let mut stones = lines[0]
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let p1_n = 25;
    let p2_n = 75;

    blink(&mut stones, p1_n);
    let partone = stones.len() as i64;
    let parttwo = 0;
    (partone, parttwo)
}

fn blink(mut stones: &mut Vec<u64>, n: i32) {
    for _ in 0..n {
        blink_once(&mut stones)
    }
}

fn blink_once(stones: &mut Vec<u64>) {
    for i in 0..stones.len() {
        let stone = stones[i];
        if stone == 0 {
            stones[i] = 1;
        } else if (stone.ilog10() % 2) == 1 {
            let (first_half, second_half) = split_number(stone);
            stones[i] = first_half;
            stones.push(second_half);
        } else {
            stones[i] *= 2024;
        }
    }
}

fn split_number(stone: u64) -> (u64, u64) {
    let num_digits = stone.ilog10() + 1;
    let first_half = stone / (10_u64).pow(num_digits / 2);
    let second_half = stone % (10_u64).pow(num_digits / 2);
    (first_half, second_half)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_11_small() {
        let lines = read_testfile("day11test.txt");
        assert_eq!(day11(lines), (55312, 0));
    }

    #[test]
    fn test_day_11() {
        let lines = read_testfile("day11.txt");
        assert_eq!(day11(lines), (203228, 0));
    }

    #[test]
    fn test_blink_once() {
        let mut stones = vec![0, 1, 10, 99, 999];
        blink_once(&mut stones);
        assert_eq!(stones, vec![1, 2024, 1, 0, 9, 9, 2021976]);
    }

    #[test]
    fn test_split_number() {
        assert_eq!(split_number(1234), (12, 34));
        assert_eq!(split_number(123456), (123, 456));
    }
}
