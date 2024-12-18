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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

use clap::Parser;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser)]
struct Cli {
    day: i64,
    path: PathBuf,
}

#[derive(Debug, Eq, PartialEq)]
enum FunctionOutput {
    IntPair(i64, i64),
    StringPair(String, String),
}

impl Display for FunctionOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionOutput::IntPair(a, b) => write!(f, "{}, {}", a, b),
            FunctionOutput::StringPair(a, b) => write!(f, "{}, {}", a, b),
        }
    }
}

fn main() {
    let args: Cli = Cli::parse();
    let lines = read_lines_from_file(args.path);
    let before = Instant::now();
    let day: fn(Vec<String>) -> FunctionOutput = match args.day {
        1 => day01::day01,
        2 => day02::day02,
        3 => day03::day03,
        4 => day04::day04,
        5 => day05::day05,
        6 => day06::day06,
        7 => day07::day07,
        8 => day08::day08,
        9 => day09::day09,
        10 => day10::day10,
        11 => day11::day11,
        12 => day12::day12,
        13 => day13::day13,
        14 => day14::day14,
        15 => day15::day15,
        16 => day16::day16,
        17 => day17::day17,
        _ => panic!(),
    };
    println!("{}", day(lines));
    println!("Elapsed time: {:.2?}", before.elapsed());
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
