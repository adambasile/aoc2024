type FileSystem = Vec<Option<usize>>;

fn filesystem_to_string(fs: &FileSystem) -> String {
    let mut out = String::new();
    for i in fs {
        match i {
            Some(i) => out.push_str(&i.to_string()),
            None => out.push_str("."),
        }
    }
    out
}

fn compress_file_blocks(filesystem: &mut FileSystem) {
    let mut front = 0;
    let mut back = filesystem.len() - 1;
    loop {
        while filesystem[front].is_some() {
            front = front + 1
        }
        while filesystem[back].is_none() {
            back = back - 1;
        }
        if front >= back {
            break;
        }
        filesystem[front] = filesystem[back];
        filesystem[back] = None;
    }
}

fn checksum(filesystem: &Vec<Option<usize>>) -> usize {
    filesystem
        .iter()
        .enumerate()
        .filter(|(_, n)| n.is_some())
        .map(|(i, n)| i * n.unwrap())
        .sum::<usize>()
}

pub(crate) fn day09(lines: Vec<String>) -> (i64, i64) {
    let orig_filesystem = parse(&lines[0]);

    let mut filesystem = orig_filesystem.clone();
    compress_file_blocks(&mut filesystem);

    let checksum = checksum(&filesystem);

    let partone = checksum as i64;
    let parttwo = 0;
    (partone, parttwo)
}

fn parse(disk_map: &str) -> FileSystem {
    let mut out = Vec::new();
    for (i, n) in disk_map
        .chars()
        .map(|n| n.to_digit(10).unwrap())
        .enumerate()
    {
        for _ in 0..n {
            out.push(match i % 2 {
                0 => Some(i / 2),
                1 => None,
                _ => panic!(),
            })
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_09_small() {
        let lines = read_testfile("day09test.txt");
        assert_eq!(day09(lines), (1928, 0));
    }

    #[test]
    fn test_day_09() {
        let lines = read_testfile("day09.txt");
        assert_eq!(day09(lines), (6259790630969, 0));
    }

    #[test]
    fn test_parse() {
        assert_eq!(filesystem_to_string(&parse("12345")), "0..111....22222")
    }
}
