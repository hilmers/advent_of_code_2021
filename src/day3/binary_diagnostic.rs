use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn solve() -> i32 {
    // File hosts must exist in current path before this produces output
    if let Ok(reader) = read_lines("./src/day2/input.txt") {
        let re = Regex::new(r"([forward|down|up]+) ([0-9]+)").unwrap();
    }
    return 1337;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

