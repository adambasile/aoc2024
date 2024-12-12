pub(crate) fn day12(lines: Vec<String>) -> (i64, i64) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_12_small() {
        let lines = read_testfile("day12test.txt");
        assert_eq!(day12(lines), (1930, 0));
    }

    #[test]
    fn test_day_12() {
        let lines = read_testfile("day12.txt");
        assert_eq!(day12(lines), (0, 0));
    }
}
