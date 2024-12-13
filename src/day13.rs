pub(crate) fn day13(lines: Vec<String>) -> (i64, i64) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_13_small() {
        let lines = read_testfile("day13test.txt");
        assert_eq!(day13(lines), (480, 0));
    }

    #[test]
    fn test_day_13() {
        let lines = read_testfile("day13.txt");
        assert_eq!(day13(lines), (0, 0));
    }
}
