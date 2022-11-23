use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    let vec = inp::parse_file("day05.txt");
    part1(&vec);
    part2(&vec);
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32
}

struct Line {
    p1: Point,
    p2: Point
}

fn parse_lines(vec: &Vec<String>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    for input_line in vec {
        let points = input_line.split(" -> ").collect::<Vec<&str>>();
        let mut p1_split = points[0].split(",");
        let p1 = Point { x: p1_split.next().expect("Error getting next split val")
                                        .parse::<u32>().expect(&"Error parsing as u32"), 
                                y: p1_split.next().expect("Error getting next split val")
                                        .parse::<u32>().expect(&"Error parsing as u32")}; 
            

        let mut p2_split = points[1].split(",");
        let p2 = Point { x: p2_split.next().expect("Error getting next split val")
                                        .parse::<u32>().expect(&"Error parsing as u32"), 
                                y: p2_split.next().expect("Error getting next split val")
                                        .parse::<u32>().expect(&"Error parsing as u32")}; 
        let line = Line { p1: p1, p2: p2};
        lines.push(line);
    }
    lines
}

fn is_horizontal_or_vertical(line: &Line) -> bool {
    line.p1.x == line.p2.x || line.p1.y == line.p2.y
}

// Solution for part 1
fn part1(vec: &Vec<String>) {
    let lines = parse_lines(vec);
    let mut field_map: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        if is_horizontal_or_vertical(&line) {
            let x1 = std::cmp::min(line.p1.x, line.p2.x);
            let x2 = std::cmp::max(line.p1.x, line.p2.x);
            let y1 = std::cmp::min(line.p1.y, line.p2.y);
            let y2 = std::cmp::max(line.p1.y, line.p2.y);
            let mut x_it = x1;
            let mut y_it = y1;
            loop {
                *field_map.entry(Point{ x: x_it, y: y_it}).or_insert(0) += 1;

                //println!("{}, {} = {}", x_it, y_it, field_map.get(&Point{x:x_it, y:y_it}).unwrap());
                if x_it == x2 && y_it == y2  { break;}
                if x_it < x2  { x_it += 1;}
                if y_it < y2  { y_it += 1;}
            }
        }
    }
    let mut spaces_greater_than_1 = 0;
    for entry in field_map {
        if entry.1 >= 2 {
            spaces_greater_than_1 += 1;
            //println!("{}, {} = {}", entry.0.x, entry.0.y, entry.1);
        }
    }
    println!("Part 1: {}", spaces_greater_than_1);
}

// Solution for part 2
fn part2(vec: &Vec<String>) {
    let lines = parse_lines(vec);
    let mut field_map: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        let x_dir = if line.p2.x as i32 - line.p1.x as i32 > 0 {1} else{-1};
        let y_dir = if line.p2.y as i32 - line.p1.y as i32 > 0 {1} else{-1};
        let x1 = line.p1.x;
        let x2 = line.p2.x;
        let y1 = line.p1.y;
        let y2 = line.p2.y;
        let mut x_it = x1 as i32;
        let mut y_it = y1 as i32;
        loop {
            *field_map.entry(Point{ x: x_it as u32, y: y_it as u32}).or_insert(0) += 1;

            //println!("{}, {} = {}", x_it, y_it, field_map.get(&Point{x:x_it as u32, y:y_it as u32}).unwrap());
            //let t = time::Duration::from_secs(1);
            //std::thread::sleep(t);
            if x_it as u32 == x2 && y_it as u32 == y2  { break;}
            if x_it as u32 != x2  { x_it += x_dir;}
            if y_it as u32 != y2  { y_it += y_dir;}
        }
    }
    let mut spaces_greater_than_1 = 0;
    for entry in field_map {
        if entry.1 >= 2 {
            spaces_greater_than_1 += 1;
            //println!("{}, {} = {}", entry.0.x, entry.0.y, entry.1);
        }
    }
    println!("Part 2: {}", spaces_greater_than_1);
}