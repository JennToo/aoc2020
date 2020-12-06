use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?)
}

pub fn group_lines_by_blank<'a>(lines: &'a [String]) -> Vec<Vec<&'a str>> {
    let mut result = Vec::new();
    let mut item = Vec::new();

    for line in lines.iter() {
        if line == "" {
            result.push(item);
            item = Vec::new();
        } else {
            item.push(line.as_str());
        }
    }
    result.push(item);

    result
}

#[macro_export]
macro_rules! advent_main {
    ($impl:ident, $data:expr) => {
        fn main() {
            let lines = aoc2020::read_lines($data).expect("Failed to read data");
            let (part1, part2) = $impl(lines.as_slice());
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        }
    };
}
