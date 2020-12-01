use itertools::Itertools;
use std::convert::TryInto;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents = std::fs::read_to_string(inputfile.to_string())
        .expect("Something went wrong reading the file");

    return contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .combinations(2)
        .filter_map(
            |combination| match combination.iter().fold(0, |sum, &x| sum + x) {
                2020usize => Some(
                    combination
                        .iter()
                        .fold(1, |product, &entry| product * entry),
                ),
                _ => None,
            },
        )
        .next()
        .unwrap();
}

fn solve_part2(inputfile: String) -> usize {
    let contents = std::fs::read_to_string(inputfile.to_string())
        .expect("Something went wrong reading the file");

    return contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .combinations(3)
        .filter_map(
            |combination| match combination.iter().fold(0, |sum, &x| sum + x) {
                2020usize => Some(
                    combination
                        .iter()
                        .fold(1, |product, &entry| product * entry),
                ),
                _ => None,
            },
        )
        .next()
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
