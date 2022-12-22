use anyhow::Result;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_file_to_string(filename: &str) -> Result<String> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}
