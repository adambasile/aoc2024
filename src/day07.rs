pub(crate) fn day07(lines: Vec<String>) -> (i32, i32) {
    let partone = 0;
    let parttwo = 0;
    (partone, parttwo)
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_07_small() {
        let lines = read_testfile("day07test.txt");
        assert_eq!(day07(lines), (3749, 0));
    }

    #[test]
    fn test_day_07() {
        let lines = read_testfile("day07.txt");
        assert_eq!(day07(lines), (0, 0));
    }
}
