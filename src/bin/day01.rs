use std::vec::Vec;

fn main() {
    let vec = inp::parse_file("day01.txt").iter()
        .map(|x| x.parse().expect(&format!("Error parsing {} as int.",x))).collect::<Vec<i32>>();
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(vec: &Vec<i32>) {
    let mut total = 0;
    let mut prev = i32::MAX;
    for curr in vec {
        if curr > &prev {
            total+=1;
        }
        prev = *curr;
    }
    println!("Part 1: {}", total);
}

// Solution for part 2
fn part2(vec: &Vec<i32>) {
    if vec.len() < 3 {
        println!("Part 2: Error, not enough input");
        return;
    }
    let mut total = 0;
    let mut window1 = vec[0] + vec[1] + vec[2];
    for i in 3..vec.len() {
        let window2 = window1 - vec[i-3] + vec[i];
        if  window2 > window1 {
            total+=1;
        }
        window1 = window2;
    }
    println!("Part 2: {}", total);
}