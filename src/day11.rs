pub(crate) fn day11(lines: Vec<String>) -> (i64, i64) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
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
        assert_eq!(day11(lines), (0, 0));
    }
}
