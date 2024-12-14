pub(crate) fn day14(lines: Vec<String>) -> (i64, i64) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_14_small() {
        let lines = read_testfile("day14test.txt");
        assert_eq!(day14(lines), (0, 0));
    }

    #[test]
    fn test_day_14() {
        let lines = read_testfile("day14.txt");
        assert_eq!(day14(lines), (0, 0));
    }
}
