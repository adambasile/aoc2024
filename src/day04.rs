pub(crate) fn day04(lines: Vec<String>) -> (i32, i32) {
    (-1, -1)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_04_small() {
        let lines = read_testfile("day04test.txt");
        assert_eq!(day04(lines), (18, 0));
    }

    #[test]
    fn test_day_04() {
        let lines = read_testfile("day04.txt");
        assert_eq!(day04(lines), (0, 0));
    }
}
