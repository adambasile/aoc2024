pub(crate) fn day10(lines: Vec<String>) -> (i64, i64) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_10_small() {
        let lines = read_testfile("day10test.txt");
        assert_eq!(day10(lines), (36, 0));
    }

    #[test]
    fn test_day_10() {
        let lines = read_testfile("day10.txt");
        assert_eq!(day10(lines), (0, 0));
    }
}
