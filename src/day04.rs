use regex::Regex;

pub(crate) fn day04(lines: Vec<String>) -> (i32, i32) {
    let re = Regex::new(r"XMAS").unwrap();
    let partone = create_candidates(&lines)
        .iter()
        .map(|cand| re.find_iter(&cand).count() as i32)
        .sum();
    (partone, -1)
}

fn create_candidates(lines: &Vec<String>) -> Vec<String> {
    let mut candidates: Vec<_> = Vec::new();
    // left to right
    for line in lines {
        candidates.push(line.into());
    }
    // top to bottom
    for i in 0..lines[0].len() {
        let mut cand = String::new();
        for line in lines {
            cand.push(line.chars().nth(i).unwrap());
        }
        candidates.push(cand);
    }

    // bottom-left to top-right
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let num_diagonals = height + width - 1;
    for diagonal in 1..=num_diagonals {
        let mut cand = String::new();
        let rows = (0..diagonal).rev();
        let cols = 0..diagonal;
        // println!("diagonal {:?}", diagonal);
        for (row, col) in rows.zip(cols) {
            if row >= height || col >= width {
                continue;
            }
            cand.push(lines[row as usize].chars().nth(col as usize).unwrap());
        }
        candidates.push(cand);
    }
    for diagonal in 1..=num_diagonals {
        let mut cand = String::new();
        let rows = (height - diagonal)..height;
        for (col, row) in rows.enumerate() {
            if row >= height || col as i32 >= width || row < 0 {
                continue;
            }
            cand.push(lines[row as usize].chars().nth(col as usize).unwrap());
        }
        candidates.push(cand);
    }
    let reversed_candidates = candidates
        .iter()
        .map(|cand| cand.chars().rev().collect())
        .collect::<Vec<_>>();
    candidates.extend(reversed_candidates);
    candidates
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_04_partone_small() {
        let lines = read_testfile("day04testpartone.txt");
        assert_eq!(day04(lines), (18, 0));
    }

    #[test]
    fn test_day_04_parttwo_small() {
        let lines = read_testfile("day04testparttwo.txt");
        assert_eq!(day04(lines), (0, 9));
    }

    #[test]
    fn test_create_candidates() {
        let cands = create_candidates(&vec!["12".to_string(), "34".to_string(), "56".to_string()]);
        assert_eq!(
            cands,
            [
                "12", "34", "56", "135", "246", "1", "32", "54", "6", "5", "36", "14", "2", "21",
                "43", "65", "531", "642", "1", "23", "45", "6", "5", "63", "41", "2"
            ]
        );
    }
}
