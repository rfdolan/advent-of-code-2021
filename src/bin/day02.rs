use std::vec::Vec;

fn main() {
    let vec = inp::parse_file("day02.txt");
    part1(&vec);
    part2(&vec);
}

fn parse_command(line: &String) -> (&str, i32) {
    let split = line.split(" ").collect::<Vec<&str>>();
    let command = split[0];
    let amount = split[1].parse::<i32>().unwrap();
    (command, amount)
}

// Solution for part 1
fn part1(vec: &Vec<String>) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in vec {
        let (command, amount) = parse_command(line);
        match command {
            "forward" => horizontal_pos += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => println!("What the hell?"),
        }
    }
    //println!("Depth: {}, horizontal: {}",depth, horizontal_pos);
    println!("Part 1: {}", horizontal_pos * depth);
}

// Solution for part 2
fn part2(vec: &Vec<String>) {
    let mut horizontal_pos = 0;
    let mut aim = 0;
    let mut depth = 0;
    for line in vec {
        let (command, amount) = parse_command(line);
        match command {
            "forward" => {
                horizontal_pos += amount;
                depth += aim * amount;
            },
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => println!("What the hell?"),
        }
    }
    //println!("Depth: {}, horizontal: {}",depth, horizontal_pos);
    println!("Part 2: {}", horizontal_pos * depth);
}