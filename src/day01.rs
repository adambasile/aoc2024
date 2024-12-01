pub(crate) fn day01(lines: Vec<String>) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_lines_from_file;

    use super::*;

    #[test]
    fn test_day_01_small() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day01test.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day01(lines), 11);
    }

    #[test]
    fn test_day_01() {
        let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("inputs")
            .join("day01.txt");
        let lines = read_lines_from_file(filename);
        assert_eq!(day01(lines), 999);
    }
}
