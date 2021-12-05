use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn solve() -> i32 {
    // File hosts must exist in current path before this produces output
    if let Ok(reader) = read_lines("./src/day2/input.txt") {
        let re = Regex::new(r"([forward|down|up]+) ([0-9]+)").unwrap();
        let mut horizontal = 0;
        let mut aim = 0;
        let mut depth = 0;
        for line in reader {
            if let Ok(dive_info) = line {
                let caps = re.captures(&dive_info).unwrap();
                let cmd = caps.get(1).unwrap().as_str();
                let val_match = caps.get(2).unwrap().as_str();
                let val = val_match.parse::<i32>().unwrap();
                if cmd == "forward" {
                    horizontal += val;
                    depth += aim * val;
                } else if cmd == "up" {
                    aim -= val;
                } else if cmd == "down" {
                    aim += val;
                }
            }
        }
        return horizontal * depth;
    }
    return 0;
}

// fn first(cmd: str, val: i32) -> i32 {
//    return 0;
// }

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

