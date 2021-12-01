use std::vec::Vec;

fn main() {
    let vec = inp::parse_file("day01.txt").iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    part1(&vec);
    part2(&vec);
}

// Solution for part 1
fn part1(vec: &Vec<i32>) {
    let mut total = 0;
    let mut iterator = vec.iter();
    let mut prev = iterator.next().unwrap();
    for curr in iterator {
        if curr > prev {
            total+=1;
        }
        prev = curr;
    }
    println!("Part 1: {}", total);
}

// Solution for part 2
fn part2(vec: &Vec<i32>) {
    let mut total = 0;
    let mut iterator = vec.iter();
    let mut first = iterator.next().unwrap();
    let mut second = iterator.next().unwrap();
    let mut third = iterator.next().unwrap();
    let mut window1 = first + second + third;
    for fourth in iterator {
        let window2 = window1 - first + fourth;
        if  window2 > window1 {
            total+=1;
        }
        first = second;
        second = third;
        third = fourth;
        window1 = window2;
    }
    println!("Part 2: {}", total);
}