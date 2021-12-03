
use std::vec::Vec;
use std::collections::HashMap;

fn main() {
    let vec = inp::parse_file("day03.txt");
    part1(&vec);
    part2(&vec);
}

// Construct map for whether or not each bit has more 1s or 0s, with a positive number meaning
// more 1s and negative being more 0s
fn construct_map(vec: &Vec<String>) -> HashMap<i32, i32> {
	let mut map : HashMap<i32, i32> = HashMap::new();
    for num in vec {
		let mut num_bits = num.len();
		for character in num.chars().collect::<Vec<char>>() {
			num_bits -= 1;
			//println!("Size {} and char {}", size, character);
			if !map.contains_key(&(num_bits as i32)) {
				map.insert(num_bits as i32, 0);
			}
			let mut val = 1;
			if character.to_digit(10).unwrap() == 0 {
				val = -1;
			}
			map.insert(num_bits as i32, map.get(&(num_bits as i32)).unwrap() + val );
		}
    }
	map
}

fn binary_to_dec(binary: &String) -> i32 {
	let mut total = 0;
	let mut bit_num = binary.len();
	for bit in binary.chars().collect::<Vec<char>>() {
		bit_num -= 1;
		if bit.to_digit(10).unwrap() != 0 {
			total += 2_i32.pow(bit_num as u32);
		}
	}
	total
}


// Solution for part 1
fn part1(vec: &Vec<String>) {
    let mut gamma = 0;
    let mut epsilon = 0;
	let bits_in_num = vec[0].len();
	let map = construct_map(vec);
	for n in 0..bits_in_num {
		let val = map.get(&(n as i32)).unwrap();
		if val >= &0 {
			gamma += 2_i32.pow(n as u32);
		}else {
			epsilon += 2_i32.pow(n as u32);
		}
	}
    println!("Part 1: {}", gamma * epsilon);
}

// Solution for part 2
fn part2(vec: &Vec<String>) {
	let mut curr_bit = vec[0].len();
	let mut numbers = vec.clone();
	let mut map = construct_map(vec);
	let bits_in_num = vec[0].len();
	while numbers.len() > 1 {
		curr_bit -= 1;
		if map.get(&(curr_bit as i32)).unwrap() >= &0 {
			numbers = numbers.iter().filter(|x| x.chars().collect::<Vec<char>>()[bits_in_num - curr_bit-1].to_digit(10).unwrap() == 1).cloned().collect::<Vec<String>>();
		} else {
			numbers = numbers.iter().filter(|x| x.chars().collect::<Vec<char>>()[bits_in_num - curr_bit-1].to_digit(10).unwrap() ==0).cloned().collect::<Vec<String>>();
		}
		// there are a lot of unnecessary ops here because instead of constructing the map I could just
		// check the val for the current character, but I'm too lazy to change it
		map = construct_map(&numbers);
	}
	let ox_rating = binary_to_dec(&numbers[0]);

	numbers = vec.clone();
	curr_bit = vec[0].len();
	while numbers.len() > 1 {
		curr_bit -= 1;
		if map.get(&(curr_bit as i32)).unwrap() < &0 {
			numbers = numbers.iter().filter(|x| x.chars().collect::<Vec<char>>()[bits_in_num - curr_bit-1].to_digit(10).unwrap() == 1).cloned().collect::<Vec<String>>();
		} else {
			numbers = numbers.iter().filter(|x| x.chars().collect::<Vec<char>>()[bits_in_num - curr_bit-1].to_digit(10).unwrap() ==0).cloned().collect::<Vec<String>>();
		}
		map = construct_map(&numbers);
	}
	let co2_rating = binary_to_dec(&numbers[0]);
	println!("Part 2: {}", ox_rating * co2_rating);
}