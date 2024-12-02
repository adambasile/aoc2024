mod day01;
mod day02;

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    day: i32,
    path: PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let lines = read_lines_from_file(args.path);
    match args.day {
        1 => println!("{:?}", day01::day01(lines)),
        2 => println!("{:?}", day02::day02(lines)),
        _ => panic!(),
    }
}

fn read_lines_from_file(path: PathBuf) -> Vec<String> {
    let file = File::open(path).unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok())
        .collect();
    lines
}

fn read_testfile(testfile: &str) -> Vec<String> {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("inputs")
        .join(testfile);
    let lines = read_lines_from_file(filename);
    lines
}
