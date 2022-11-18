use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

// Parse file with given name in parent directory into a vector of ints
pub fn parse_file(name: &str) -> Vec<String> {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(name) {
        for line in lines {
            if let Ok(ip) = line {
                vec.push(ip);
            }
        }
    }
    vec
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}