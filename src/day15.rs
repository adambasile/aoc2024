pub(crate) fn day15(lines: Vec<String>) -> (i64, i64) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_15_small() {
        let lines = read_testfile("day15test.txt");
        assert_eq!(day15(lines), (2028, 0));
    }

    #[test]
    fn test_day_15() {
        let lines = read_testfile("day15.txt");
        assert_eq!(day15(lines), (0, 0));
    }
}
