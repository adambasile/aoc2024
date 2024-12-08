pub(crate) fn day08(lines: Vec<String>) -> (i64, i64) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_08_small() {
        let lines = read_testfile("day08test.txt");
        assert_eq!(day08(lines), (14, 0));
    }

    #[test]
    fn test_day_08() {
        let lines = read_testfile("day08.txt");
        assert_eq!(day08(lines), (0, 0));
    }
}
