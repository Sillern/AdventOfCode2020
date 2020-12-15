use std::collections::HashMap;
use std::env;

fn solve_parts(starting_numbers: Vec<usize>, count: usize) -> usize {
    let mut memory = HashMap::<usize, (usize, usize)>::new();

    let mut last_spoken = 0;
    starting_numbers
        .iter()
        .enumerate()
        .for_each(|(index, number)| {
            memory.entry(*number).or_insert((index, index));
            last_spoken = *number;
        });

    for turn in starting_numbers.len()..count {
        let mut update_entry = false;
        match memory.get(&last_spoken) {
            Some((previously_been_spoken, last_been_spoken)) => {
                last_spoken = last_been_spoken - previously_been_spoken;
                update_entry = true;
            }
            None => {
                println!("Has not spoken {} before", last_spoken);
            }
        }

        if update_entry {
            memory
                .entry(last_spoken)
                .and_modify(|entry| {
                    let (_, last_been_spoken) = *entry;
                    *entry = (last_been_spoken, turn);
                })
                .or_insert((turn, turn));
        }
    }
    last_spoken
}

fn main() {
    println!("Example1: {}", solve_parts(vec![0, 3, 6], 2020));
    println!("Example2: {}", solve_parts(vec![2, 1, 3], 2020));
    println!("Example3: {}", solve_parts(vec![1, 2, 3], 2020));
    println!("Example4: {}", solve_parts(vec![2, 3, 1], 2020));
    println!("Example5: {}", solve_parts(vec![3, 2, 1], 2020));
    println!("Example6: {}", solve_parts(vec![3, 1, 2], 2020));
    println!("Part1: {}", solve_parts(vec![2, 1, 10, 11, 0, 6], 2020));
    println!("Example1, part2: {}", solve_parts(vec![0, 3, 6], 30000000));
    println!("Part2: {}", solve_parts(vec![2, 1, 10, 11, 0, 6], 30000000));
}
