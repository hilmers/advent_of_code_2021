use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        first(lines);
    }
}

fn second(lines: io::Lines<io::BufReader<File>>) {
    // rulla po 3
}
fn first(lines: io::Lines<io::BufReader<File>>) {
    // Consumes the iterator, returns an (Optional) String
    let mut last_measurement = i32::MAX;
    let mut increased_measurement = 0;
    for line in lines {
        if let Ok(ip) = line {
            let measurement = ip.parse::<i32>().unwrap();
            if measurement > last_measurement {
                increased_measurement += 1;
            } 
            last_measurement = measurement;
        }
    }
    println!("Increased measurements: {}", increased_measurement);

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

