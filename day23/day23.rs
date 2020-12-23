use itertools::Itertools;
use std::collections::VecDeque;
use std::env;

fn print_cups(cups: &Vec<usize>, current_cup: &usize, print_length: usize) {
    print!("cups: ");
    let mut printed_values = 0;
    let mut cursor = current_cup;
    for index in 1..cups.len() {
        let cup = cursor;
        if cup == current_cup {
            print!("({})", cup);
        } else {
            print!(" {} ", cup);
        }
        cursor = &cups[*cursor];

        printed_values += 1;
        if printed_values > print_length {
            break;
        }
    }
    println!();
}

fn make_move(cups: &mut Vec<usize>, current_cup: &mut usize) {
    let taken_cups = vec![
        cups[*current_cup],
        cups[cups[*current_cup]],
        cups[cups[cups[*current_cup]]],
    ];

    let next_cup = cups[*taken_cups.last().unwrap()];
    let mut destination = *current_cup - 1;
    while taken_cups.contains(&destination) || destination == 0 {
        destination = (destination + cups.len() - 1) % cups.len();
    }

    cups[*current_cup] = next_cup;

    let insert_before = cups[destination];
    cups[destination] = taken_cups[0];
    cups[taken_cups[0]] = taken_cups[1];
    cups[taken_cups[1]] = taken_cups[2];
    cups[taken_cups[2]] = insert_before;

    *current_cup = next_cup;
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut current_cup: usize = 0;

    let pairs = contents
        .chars()
        .enumerate()
        .peekable()
        .batching(|it| match it.next() {
            None => None,
            Some((index, c)) => {
                if c.is_numeric() {
                    let digit = c.to_digit(10).unwrap() as usize;

                    if current_cup == 0 {
                        current_cup = digit;
                    }

                    Some(match it.peek() {
                        Some((index, next_c)) => {
                            let next_digit = if next_c.is_numeric() {
                                next_c.to_digit(10).unwrap() as usize
                            } else {
                                current_cup
                            };
                            (digit, next_digit)
                        }
                        None => (digit, current_cup),
                    })
                } else {
                    None
                }
            }
        })
        .collect::<Vec<(usize, usize)>>();

    let mut cups = (0..(pairs.len() + 1)).collect::<Vec<usize>>();
    println!("pairs: {:?}", pairs);
    println!("cups: {:?}", cups);
    pairs
        .iter()
        .for_each(|(value, next_value)| cups[*value] = *next_value);
    println!("cups: {:?}", cups);

    for move_count in 0..100 {
        println!("-- move {} --", move_count + 1);
        //print_cups(&cups, &current_cup);
        make_move(&mut cups, &mut current_cup);
    }

    println!("-- final --");
    print_cups(&cups, &1, 10);
    0
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut current_cup: usize = 0;

    let pairs = contents
        .chars()
        .enumerate()
        .peekable()
        .batching(|it| match it.next() {
            None => None,
            Some((index, c)) => {
                if c.is_numeric() {
                    let digit = c.to_digit(10).unwrap() as usize;

                    if current_cup == 0 {
                        current_cup = digit;
                    }

                    Some(match it.peek() {
                        Some((index, next_c)) => {
                            let next_digit = if next_c.is_numeric() {
                                next_c.to_digit(10).unwrap() as usize
                            } else {
                                10
                            };
                            (digit, next_digit)
                        }
                        None => (digit, 10),
                    })
                } else {
                    None
                }
            }
        })
        .collect::<Vec<(usize, usize)>>();

    let mut cups = (0..1000001)
        .map(|value| value + 1 as usize)
        .collect::<Vec<usize>>();
    let last_value = cups.len() - 1;
    cups[last_value] = current_cup.clone();

    pairs
        .iter()
        .for_each(|(value, next_value)| cups[*value] = *next_value);

    for move_count in 0..10000000 {
        if move_count % 500000 == 0 {
            println!("-- move {} --", move_count + 1);
        }
        //print_cups(&cups, &current_cup);
        make_move(&mut cups, &mut current_cup);
    }

    println!("-- final --");
    print_cups(&cups, &1, 10);
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
