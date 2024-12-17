use crate::FunctionOutput;
use crate::FunctionOutput::IntPair;

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

#[derive(Copy, Clone)]
struct File {
    value: Option<usize>,
    size: usize,
}

fn checksum(filesystem: &Vec<Option<usize>>) -> usize {
    filesystem
        .iter()
        .enumerate()
        .filter(|(_, n)| n.is_some())
        .map(|(i, n)| i * n.unwrap())
        .sum::<usize>()
}

fn compress_files(diskmap: &mut Vec<File>) {
    for file_idx in (0..diskmap.len()).rev() {
        if diskmap[file_idx].value.is_none() {
            continue;
        }
        let file_size = diskmap[file_idx].size;
        for gap in 0..file_idx {
            if diskmap[gap].value.is_some() || diskmap[gap].size < file_size {
                continue;
            }
            diskmap[gap].size -= diskmap[file_idx].size;
            let new_file = diskmap[file_idx].clone();
            diskmap[file_idx].value = None;
            diskmap.insert(gap, new_file);
            break;
        }
    }
}

pub(crate) fn day09(lines: Vec<String>) -> FunctionOutput {
    let mut diskmap = parse_diskmap(&lines[0]);
    let mut filesystem = diskmap_to_filesystem(&diskmap);
    compress_file_blocks(&mut filesystem);
    compress_files(&mut diskmap);
    let partone = checksum(&filesystem) as i64;
    let parttwo = checksum(&diskmap_to_filesystem(&diskmap)) as i64;
    IntPair(partone, parttwo)
}

fn parse_diskmap(disk_map: &str) -> Vec<File> {
    disk_map
        .chars()
        .map(|n| n.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(i, n)| File {
            value: match i % 2 {
                0 => Some(i / 2),
                1 => None,
                _ => panic!(),
            },
            size: n,
        })
        .collect()
}

fn diskmap_to_filesystem(diskmap: &Vec<File>) -> FileSystem {
    diskmap
        .iter()
        .map(|file| (0..file.size).map(|_| file.value))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::read_testfile;

    use super::*;

    #[test]
    fn test_day_09_small() {
        let lines = read_testfile("day09test.txt");
        assert_eq!(day09(lines), IntPair(1928, 2858));
    }

    #[test]
    fn test_day_09() {
        let lines = read_testfile("day09.txt");
        assert_eq!(day09(lines), IntPair(6259790630969, 6289564433984));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            filesystem_to_string(&diskmap_to_filesystem(&parse_diskmap("12345"))),
            "0..111....22222"
        )
    }
}
