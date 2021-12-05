use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Increased measurements: {}", solve());
}

pub fn solve() -> i32 {
    // File hosts must exist in current path before this produces output
    if let Ok(reader) = read_lines("./src/day1/input.txt") {
        let lines_v: Vec<_> = reader.collect::<Result<_, _>>().unwrap();
        return second(lines_v);
    } else {
        println!("Couldn't read input file");
        return 0;
    }
}

fn second(lines: Vec<String>) -> i32 {
    let mut increased_measurement = 0;
    for n in 2..lines.len()-1 {
        let a1 = lines[n-2].parse::<i32>().unwrap();
        let a2 = lines[n-1].parse::<i32>().unwrap();
        let a3 = lines[n].parse::<i32>().unwrap();
        let b1 = lines[n-1].parse::<i32>().unwrap();
        let b2 = lines[n].parse::<i32>().unwrap();
        let b3 = lines[n+1].parse::<i32>().unwrap();
        if (b1+b2+b3) > (a1+a2+a3) {
            increased_measurement += 1;
        }
    }
    return increased_measurement;
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

