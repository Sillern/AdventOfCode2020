use itertools::Itertools;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let timestamp = contents.lines().nth(0).unwrap().parse::<usize>().unwrap();

    let (lowest_waiting_time, bus_id) = contents
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .filter_map(|bus_id| match bus_id {
            "x" => None,
            _ => Some(bus_id.parse::<usize>().unwrap()),
        })
        .fold((timestamp, 0), |(lowest_waiting_time, id), bus_id| {
            let current_waiting_time = bus_id - (timestamp % bus_id);
            if current_waiting_time < lowest_waiting_time {
                (current_waiting_time, bus_id)
            } else {
                (lowest_waiting_time, id)
            }
        });

    lowest_waiting_time * bus_id
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let bus_ids = contents
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(offset, bus_id)| match bus_id {
            "x" => None,
            _ => Some((offset, bus_id.parse::<usize>().unwrap())),
        })
        .collect::<Vec<(usize, usize)>>();

    println!("bus_ids: {:?}", bus_ids);
    let mut timestamp = 0;

    for num_buses in 0..bus_ids.len() {
        let increment = bus_ids
            .iter()
            .take(num_buses)
            .fold(1, |product, (_, bus_id)| product * bus_id);

        loop {
            if bus_ids
                .iter()
                .take(num_buses + 1)
                .all(|(offset, bus_id)| (timestamp + *offset) % bus_id == 0)
            {
                println!("Increment is now: {}, timestamp: {}", increment, timestamp);
                break;
            }
            timestamp += increment;
            println!("multiple: {}, timestamp: {}", increment, timestamp);
        }
    }
    timestamp
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
