use itertools::Itertools;
use std::collections::VecDeque;
use std::env;

fn print_cups(cups: &VecDeque<u32>, current_cup: &u32) {
    print!("cups: ");
    cups.iter().for_each(|cup| {
        if cup == current_cup {
            print!("({})", cup);
        } else {
            print!(" {} ", cup);
        }
    });
    println!();
}

fn make_move(cups: &mut VecDeque<u32>, current_cup: &mut u32) {
    let current_cup_index = cups
        .iter()
        .find_position(|&cup| *cup == *current_cup)
        .unwrap()
        .0;

    let taken_cups = (0..3)
        .map(|_| {
            let remove_index = if current_cup_index + 1 >= cups.len() {
                0
            } else {
                current_cup_index + 1
            };
            cups.remove(remove_index).unwrap()
        })
        .collect::<VecDeque<u32>>();

    let remove_index = if current_cup_index + 1 >= cups.len() {
        0
    } else {
        current_cup_index + 1
    };
    let next_cup = cups[remove_index].clone();

    let max_cup_value = *cups.iter().max().unwrap() as u32;

    let mut next_cup_index_maybe = None;
    while next_cup_index_maybe == None {
        *current_cup = (*current_cup + (max_cup_value + 1) - 1) % (max_cup_value + 1);
        next_cup_index_maybe = cups.iter().find_position(|&cup| *cup == *current_cup);
    }

    let next_cup_index = next_cup_index_maybe.unwrap().0;

    taken_cups
        .iter()
        .enumerate()
        .for_each(|(cup_index, cup)| cups.insert(next_cup_index + 1 + cup_index, *cup));
    *current_cup = next_cup;
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut cups = contents
        .chars()
        .filter_map(|c| {
            if c.is_numeric() {
                Some(c.to_digit(10).unwrap())
            } else {
                None
            }
        })
        .collect::<VecDeque<u32>>();

    let mut current_cup = cups.front().unwrap().clone();

    for move_count in 0..100 {
        println!("-- move {} --", move_count + 1);
        make_move(&mut cups, &mut current_cup);
    }
    println!("-- final --");
    print_cups(&cups, &current_cup);
    0
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut cups = contents
        .chars()
        .filter_map(|c| {
            if c.is_numeric() {
                Some(c.to_digit(10).unwrap())
            } else {
                None
            }
        })
        .collect::<VecDeque<u32>>();

    ((cups.len() + 1)..1000000).for_each(|value| cups.push_back(value as u32));

    let mut current_cup = cups.front().unwrap().clone();

    for move_count in 0..10000000 {
        println!("-- move {} --", move_count + 1);
        make_move(&mut cups, &mut current_cup);
    }
    println!("-- final --");
    print_cups(&cups, &current_cup);
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
