use regex::Regex;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let pattern =
        Regex::new(r"(mask\s=\s(?P<mask>[X01]+)$)|(mem\[(?P<address>\d+)]\s=\s(?P<value>\d+)$)")
            .unwrap();

    let mut memory = HashMap::<usize, usize>::new();

    let mut bitmask_and = 0xFFFFFFFFFF as usize;
    let mut bitmask_or = 0x0 as usize;

    let instructions = contents.lines().for_each(|line| {
        let parsed = pattern.captures(line).unwrap();

        match parsed.name("address") {
            Some(_) => {
                let address = parsed["address"].parse::<usize>().unwrap();
                let mut value = parsed["value"].parse::<usize>().unwrap();

                // apply bitmasks
                value &= bitmask_and;
                value |= bitmask_or;

                memory
                    .entry(address)
                    .and_modify(|e| *e = value)
                    .or_insert(value);
            }
            _ => {
                bitmask_and = 0xFFFFFFFFFF as usize;
                bitmask_or = 0x0 as usize;

                parsed["mask"]
                    .chars()
                    .enumerate()
                    .for_each(|(index, bits)| {
                        match bits {
                            'X' => {}
                            '1' => {
                                bitmask_or |= 1 << (35 - index);
                            }
                            '0' => {
                                bitmask_and ^= 1 << (35 - index);
                            }
                            _ => {
                                println!("Unknown bitpattern");
                            }
                        };
                    });
            }
        };
    });

    memory.values().fold(0, |sum, value| sum + value)
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let pattern =
        Regex::new(r"(mask\s=\s(?P<mask>[X01]+)$)|(mem\[(?P<address>\d+)]\s=\s(?P<value>\d+)$)")
            .unwrap();

    let mut memory = HashMap::<usize, usize>::new();
    let mut bitmask_and = 0x0 as usize;
    let mut bitmasks = Vec::<usize>::new();

    let instructions = contents.lines().for_each(|line| {
        let parsed = pattern.captures(line).unwrap();

        match parsed.name("address") {
            Some(_) => {
                let address = parsed["address"].parse::<usize>().unwrap();
                let value = parsed["value"].parse::<usize>().unwrap();

                bitmasks.iter().for_each(|bitmask| {
                    memory
                        .entry((address & bitmask_and) | bitmask)
                        .and_modify(|e| *e = value)
                        .or_insert(value);
                });
            }
            _ => {
                bitmasks.clear();
                bitmasks.push(0x0);
                bitmask_and = 0x0;

                parsed["mask"]
                    .chars()
                    .enumerate()
                    .for_each(|(index, bits)| {
                        match bits {
                            'X' => {
                                let mut bitmasks_upper = bitmasks
                                    .iter()
                                    .map(|bitmask| *bitmask | 1 << (35 - index))
                                    .collect::<Vec<usize>>();

                                bitmasks_upper.extend(bitmasks.clone());

                                bitmasks = bitmasks_upper;
                            }
                            '1' => {
                                bitmask_and |= (1 << (35 - index));
                                bitmasks = bitmasks
                                    .iter()
                                    .map(|bitmask| *bitmask | 1 << (35 - index))
                                    .collect::<Vec<usize>>();
                            }
                            '0' => {
                                bitmask_and |= (1 << (35 - index));
                            }
                            _ => {
                                println!("Unknown bitpattern");
                            }
                        };
                    });
            }
        };
    });
    //println!("memory: {:#?}", memory);

    memory.values().fold(0, |sum, value| sum + value)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
