mod day01;

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
