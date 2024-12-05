use regex::Regex;

pub(crate) fn day04(lines: Vec<String>) -> (i32, i32) {
    let re = Regex::new(r"XMAS").unwrap();
    let partone = create_candidates(&lines)
        .iter()
        .map(|cand| re.find_iter(&cand).count() as i32)
        .sum();

    let re_parttwo = Regex::new(r"MMSS|MSSM|SSMM|SMMS").unwrap();
    let mut parttwo = 0;
    for (row, line) in (&lines).iter().enumerate() {
        let row = row as i32;
        for (col, c) in line.chars().enumerate() {
            if c != 'A' {
                continue;
            }
            let col = col as i32;
            let Some(top_left) = get_char(&lines, row - 1, col - 1) else {
                continue;
            };
            let Some(bottom_left) = get_char(&lines, row + 1, col - 1) else {
                continue;
            };
            let Some(bottom_right) = get_char(&lines, row + 1, col + 1) else {
                continue;
            };
            let Some(top_right) = get_char(&lines, row - 1, col + 1) else {
                continue;
            };
            let test: String = [top_left, bottom_left, bottom_right, top_right]
                .iter()
                .collect();
            if re_parttwo.is_match(&test) {
                parttwo += 1
            }
        }
    }
    (partone, parttwo)
}

fn get_char(lines: &Vec<String>, row: i32, col: i32) -> Option<char> {
    if row < 0 || col < 0 {
        return None;
    }
    let line = match lines.get(row as usize) {
        None => return None,
        Some(line) => line,
    };
    line.chars().nth(col as usize)
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

    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let num_diagonals = height + width - 1;
    // bottom-left to top-right
    for diagonal in 1..=num_diagonals {
        let mut cand = String::new();
        let rows = (0..diagonal).rev();
        let cols = 0..diagonal;
        for (row, col) in rows.zip(cols) {
            if row >= height || col >= width {
                continue;
            }
            cand.push(lines[row as usize].chars().nth(col as usize).unwrap());
        }
        candidates.push(cand);
    }
    // top-left to bottom-right
    for diagonal in 1..=num_diagonals {
        let mut cand = String::new();
        let rows = (height - diagonal)..height;
        for (col, row) in rows.enumerate() {
            if row >= height || col as i32 >= width || row < 0 {
                continue;
            }
            cand.push(lines[row as usize].chars().nth(col).unwrap());
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
        assert_eq!(day04(lines), (18, 9));
    }

    #[test]
    fn test_day_04_parttwo_small() {
        let lines = read_testfile("day04testparttwo.txt");
        assert_eq!(day04(lines), (0, 9));
    }

    #[test]
    fn test_day_04() {
        let lines = read_testfile("day04.txt");
        assert_eq!(day04(lines), (2685, 2048));
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
