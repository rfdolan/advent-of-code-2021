use std::vec::Vec;

const NEW_FISH_DAYS_UNTIL_SPAWN: u32 = 9;
const DAYS_UNTIL_SPAWN: u32 = 7;

fn main() {
    let line = &inp::parse_file("day06.txt")[0];
    let vec = line.split(",")
            .map(|x| x.parse::<u32>()
                    .expect(&format!("Could not parse {} as u32.", x)))
            .collect::<Vec<u32>>();
    part1(&vec);
    part2(&vec);
}

fn do_sim(vec: &Vec<u32>, cycles: u32) -> u64 {
    let mut final_answer: u64 = 0;
    let mut fish_born_at_cycle: Vec<u64> = vec![0; (cycles+1) as usize];
    final_answer += vec.len() as u64;  // How many fish we start with.

    // For each fish go through and add a fish to our array each time they would spawn one.
    for fish in vec {
        let mut future_cycle = *fish + 1;
        while future_cycle <= cycles  {
            fish_born_at_cycle[future_cycle as usize] += 1;
            future_cycle += DAYS_UNTIL_SPAWN;
        }
    }

    for current_cycle in 1..cycles+1 {
        let mut fish_to_add = 0;
        match fish_born_at_cycle.get(current_cycle as usize) {
            // If there are fish to spawn this cycle, spawn them and compute what fish they will make.
            Some(num_fish_born) => {
                fish_to_add = *num_fish_born;
            },
            None => ()
        }

        let mut future_cycle = current_cycle + NEW_FISH_DAYS_UNTIL_SPAWN;
        while future_cycle <= cycles  {
            fish_born_at_cycle[future_cycle as usize] += fish_to_add;
            future_cycle += DAYS_UNTIL_SPAWN;
        }

        //println!("Cycle {}: add {} fish", current_cycle, fish_to_add);
        final_answer += fish_to_add as u64; // Add the number of fish born this step.
    }

    final_answer

}

// Solution for part 1
fn part1(vec: &Vec<u32>) {
    println!("Part 1: {}", do_sim(vec, 80));
}

// Solution for part 2
fn part2(vec: &Vec<u32>) {
    println!("Part 2: {}", do_sim(vec, 256));
}