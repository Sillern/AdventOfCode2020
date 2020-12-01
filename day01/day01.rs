use itertools::Itertools;
use std::convert::TryInto;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents = std::fs::read_to_string(inputfile.to_string())
        .expect("Something went wrong reading the file");

    let entries = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    return entries
        .iter()
        .filter_map(|x| entries.iter().find(|&&y| y == 2020usize - *x))
        .fold(1, |product, &entry| product * entry);
}

fn solve_part2(inputfile: String) -> usize {
    let contents = std::fs::read_to_string(inputfile.to_string())
        .expect("Something went wrong reading the file");

    let entries = contents
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    return entries
        .iter()
        .filter_map(|x| entries.iter().find(|&&y| y == 2020usize - *x))
        .fold(1, |product, &entry| product * entry);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
