pub(crate) fn day16(lines: Vec<String>) -> (i64, i64) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::day16::day16;
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_16_small1() {
        let lines = read_testfile("day16test1.txt");
        assert_eq!(day16(lines), (7036, 0));
    }
    #[test]
    fn test_day_16_small2() {
        let lines = read_testfile("day16test2.txt");
        assert_eq!(day16(lines), (11048, 0));
    }
    #[test]
    fn test_day_16() {
        let lines = read_testfile("day16.txt");
        assert_eq!(day16(lines), (0, 0));
    }
}
