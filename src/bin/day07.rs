use std::vec::Vec;

use regex::bytes::CaptureLocations;

fn main() {
    let line = &inp::parse_file("day07.txt")[0];
    let vec = line.split(",")
            .map(|x| x.parse::<u32>()
                    .expect(&format!("Could not parse {} as u32.", x)))
            .collect::<Vec<u32>>();
    part1(&vec);
    part2(&vec);
}

fn get_mode(vec: &Vec<u32>) -> u32 {
    let mut sorted = vec.to_vec();
    sorted.sort();
    let mut max_run = 0;
    let mut max_num = 0;
    let mut curr_num: u32 = 0;
    let mut curr_run = 0;
    for num in sorted.iter() {
        if *num == curr_num {
            curr_run += 1;
        } else {
            if curr_run > max_run {
                max_run = curr_run;
                max_num = curr_num;
            }
            curr_run = 1;
            curr_num = *num;
        }
    }
    max_num
}

// Solution for part 1
fn part1(vec: &Vec<u32>) {
    let mode = get_mode(vec);

    let mut min_fuel_used = u32::MAX;
    let mut curr_num = mode;

    loop {
        let mut fuel_used = 0;
        for num in vec {
            let diff = (*num as i32 - curr_num as i32).abs();
            fuel_used += diff as u32;
        }
        if fuel_used < min_fuel_used {
            min_fuel_used = fuel_used;
        } else {
            break;
        }
        curr_num += 1;
    }
    loop {
        let mut fuel_used = 0;
        for num in vec {
            let diff = (*num as i32 - curr_num as i32).abs();
            fuel_used += diff as u32;
        }
        if fuel_used < min_fuel_used {
            min_fuel_used = fuel_used;
        } else {
            break;
        }
        curr_num -= 1;
    }

    println!("Part 1: {}", min_fuel_used);
}

// 1-1, 2-3, 3-6, 4-10, 5-15

fn calc_cost(num: u32) -> u32 {
    let mut val = 0;
    for x in 0..num+1 {
        val += x;
    }
    val
}

// Solution for part 2
fn part2(vec: &Vec<u32>) {
    let mode = get_mode(vec);

    let mut min_fuel_used = u32::MAX;
    let mut curr_num = mode;

    loop {
        let mut fuel_used = 0;
        for num in vec {
            let diff = (*num as i32 - curr_num as i32).abs();
            fuel_used += calc_cost(diff as u32);
        }
        if fuel_used < min_fuel_used {
            min_fuel_used = fuel_used;
        } else {
            break;
        }
        curr_num += 1;
    }
    loop {
        let mut fuel_used = 0;
        for num in vec {
            let diff = (*num as i32 - curr_num as i32).abs();
            fuel_used += calc_cost(diff as u32);
        }
        if fuel_used < min_fuel_used {
            min_fuel_used = fuel_used;
        } else {
            break;
        }
        curr_num -= 1;
    }
    println!("Part 2: {}", min_fuel_used);
}