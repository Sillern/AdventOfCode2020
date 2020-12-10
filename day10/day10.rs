use itertools::Itertools;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut adapters = contents
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    adapters.sort();

    let used_adapters = Vec::<usize>::new();
    let initial_outlet = 0;

    fn find_adapter_chain(
        adapters: &Vec<usize>,
        previous_outlet: usize,
        differences: &mut HashMap<usize, usize>,
    ) -> usize {
        let jolt_difference = 3;
        match adapters
            .iter()
            .filter_map(|&jolt| {
                if jolt > previous_outlet && jolt <= (previous_outlet + jolt_difference) {
                    differences
                        .entry(jolt - previous_outlet)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                    Some(jolt)
                } else {
                    None
                }
            })
            .next()
        {
            Some(next_adapter) => find_adapter_chain(&adapters, next_adapter, differences),
            None => previous_outlet + 3,
        }
    }
    let mut differences = HashMap::<usize, usize>::new();
    let final_adapter = find_adapter_chain(&adapters, initial_outlet, &mut differences);

    let one_diffs = differences[&1];
    let three_diffs = differences[&3] + 1;
    println!(
        "All differences: {:?}, final: {}",
        differences, final_adapter
    );

    one_diffs * three_diffs
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut adapters = contents
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    adapters.sort();

    let used_adapters = Vec::<usize>::new();
    let initial_outlet = 0;

    fn find_adapter_chain(
        adapters: &Vec<usize>,
        previous_outlet: usize,
        cache: &mut HashMap<usize, usize>,
    ) -> usize {
        let jolt_difference = 3;
        let valid_adapters = adapters.iter().filter_map(|&jolt| {
            if jolt > previous_outlet && jolt <= (previous_outlet + jolt_difference) {
                Some(jolt)
            } else {
                None
            }
        });

        let num_available_adapters = valid_adapters.clone().fold(0, |count, _| count + 1);
        if num_available_adapters == 0 {
            return 1;
        }

        valid_adapters.fold(0, |sum, next_adapter| {
            sum + if cache.contains_key(&next_adapter) {
                cache[&next_adapter]
            } else {
                let next_value = find_adapter_chain(&adapters, next_adapter, cache);
                cache.insert(next_adapter, next_value);
                next_value
            }
        })
    }

    let mut cache = HashMap::<usize, usize>::new();
    find_adapter_chain(&adapters, initial_outlet, &mut cache)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
