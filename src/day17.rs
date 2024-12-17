use crate::FunctionOutput;
use crate::FunctionOutput::StringPair;

pub(crate) fn day17(lines: Vec<String>) -> FunctionOutput {
    let partone = String::new();
    let parttwo = String::new();
    StringPair(partone, parttwo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_testfile;

    #[test]
    fn test_day_17_small() {
        let lines = read_testfile("day17test.txt");
        assert_eq!(day17(lines), StringPair("".to_string(), "".to_string()));
    }

    #[test]
    fn test_day_17() {
        let lines = read_testfile("day17.txt");
        assert_eq!(day17(lines), StringPair("".to_string(), "".to_string()));
    }
}
