use itertools::Itertools;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .combinations(2)
        .filter_map(|combination| match combination.iter().sum() {
            2020usize => Some(combination.iter().product()),
            _ => None,
        })
        .next()
        .unwrap()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .combinations(3)
        .filter_map(|combination| match combination.iter().sum() {
            2020usize => Some(combination.iter().product()),
            _ => None,
        })
        .next()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
