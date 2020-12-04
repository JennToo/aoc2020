use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file)
        .lines()
        .collect::<io::Result<Vec<String>>>()?)
}
