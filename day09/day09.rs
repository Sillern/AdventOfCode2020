use itertools::Itertools;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let preamble = 25;

    let sequence = contents
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut invalid_entry = 0;
    sequence
        .iter()
        .skip(preamble)
        .fold(preamble, |index, entry| {
            if !&sequence[index - preamble..index]
                .iter()
                .combinations(2)
                .any(|combination| combination[0] + combination[1] == *entry)
            {
                println!(
                    "invalid entry: {} not in {:?}",
                    *entry,
                    &sequence[index - preamble..index]
                );
                invalid_entry = *entry;
            }
            index + 1
        });
    invalid_entry
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let preamble = 25;

    let sequence = contents
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut invalid_entry = 0;
    sequence
        .iter()
        .skip(preamble)
        .fold(preamble, |index, entry| {
            if !&sequence[index - preamble..index]
                .iter()
                .combinations(2)
                .any(|combination| combination[0] + combination[1] == *entry)
            {
                invalid_entry = *entry;
            }
            index + 1
        });

    let mut weakness = 0;
    for start_index in 0..sequence.len() {
        sequence
            .iter()
            .enumerate()
            .skip(start_index)
            .any(|(index, entry)| {
                if index - start_index > 1 {
                    let subsequence = &sequence[start_index..index];

                    let sum = subsequence.iter().sum::<usize>();

                    if sum == invalid_entry {
                        let min = subsequence.iter().min().unwrap();
                        let max = subsequence.iter().max().unwrap();

                        weakness = min + max;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            });
    }
    weakness
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
