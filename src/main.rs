mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    day: i64,
    path: PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();
    let lines = read_lines_from_file(args.path);
    let day = match args.day {
        1 => day01::day01,
        2 => day02::day02,
        3 => day03::day03,
        4 => day04::day04,
        5 => day05::day05,
        6 => day06::day06,
        7 => day07::day07,
        8 => day08::day08,
        9 => day09::day09,
        _ => panic!(),
    };
    println!("{:?}", day(lines))
}

fn read_lines_from_file(path: PathBuf) -> Vec<String> {
    let file = File::open(path).unwrap();
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok())
        .collect();
    lines
}

#[allow(unused)]
fn read_testfile(testfile: &str) -> Vec<String> {
    let filename = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("inputs")
        .join(testfile);
    let lines = read_lines_from_file(filename);
    lines
}
