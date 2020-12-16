use regex::Regex;
use std::collections::HashMap;
use std::env;

type Range = (usize, usize);

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let rule_pattern =
        Regex::new(r"(?P<rule_name>.+):\s(?P<range1_start>\d+)-(?P<range1_end>\d+)\sor\s(?P<range2_start>\d+)-(?P<range2_end>\d+)$").unwrap();

    let mut rules = HashMap::<Range, String>::new();

    let first_block = contents.split("\n\n").nth(0).unwrap();
    let third_block = contents.split("\n\n").nth(2).unwrap();

    first_block.lines().for_each(|line| {
        let parsed = rule_pattern.captures(line).unwrap();
        let rule_name = &parsed["rule_name"];
        let range1 = (
            parsed["range1_start"].parse::<usize>().unwrap(),
            parsed["range1_end"].parse::<usize>().unwrap(),
        );
        let range2 = (
            parsed["range2_start"].parse::<usize>().unwrap(),
            parsed["range2_end"].parse::<usize>().unwrap(),
        );
        rules.entry(range1).or_insert(rule_name.to_string());
        rules.entry(range2).or_insert(rule_name.to_string());
    });

    let mut ticket_scanning_error_rate = 0;
    third_block
        .lines()
        .skip(1)
        .enumerate()
        .for_each(|(ticket_index, line)| {
            println!("ticket: {}", ticket_index);
            line.split(',').for_each(|field| {
                let value = field.parse::<usize>().unwrap();
                let is_valid = rules
                    .keys()
                    .any(|&(start, end)| value >= start && value <= end);
                if !is_valid {
                    println!("    {}", value);
                    ticket_scanning_error_rate += value;
                }
            });
        });

    ticket_scanning_error_rate
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let rule_pattern =
        Regex::new(r"(?P<rule_name>.+):\s(?P<range1_start>\d+)-(?P<range1_end>\d+)\sor\s(?P<range2_start>\d+)-(?P<range2_end>\d+)$").unwrap();

    let mut rules = HashMap::<(Range, Range), String>::new();

    let first_block = contents.split("\n\n").nth(0).unwrap();
    let second_block = contents.split("\n\n").nth(1).unwrap();
    let third_block = contents.split("\n\n").nth(2).unwrap();

    first_block.lines().for_each(|line| {
        let parsed = rule_pattern.captures(line).unwrap();
        let rule_name = &parsed["rule_name"];
        let range1 = (
            parsed["range1_start"].parse::<usize>().unwrap(),
            parsed["range1_end"].parse::<usize>().unwrap(),
        );
        let range2 = (
            parsed["range2_start"].parse::<usize>().unwrap(),
            parsed["range2_end"].parse::<usize>().unwrap(),
        );
        rules
            .entry((range1, range2))
            .or_insert(rule_name.to_string());
    });

    let my_ticket = second_block
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',').map(|field| {
                println!("field: {:?}", field);
                field.parse::<usize>().unwrap()
            })
        })
        .next()
        .unwrap()
        .collect::<Vec<usize>>();

    let mut ticket_fields = Vec::<Vec<usize>>::new();
    third_block
        .lines()
        .skip(1)
        .enumerate()
        .for_each(|(ticket_index, line)| {
            let num_fields = line.split(',').count();
            let fields = line
                .split(',')
                .filter_map(|field| {
                    let value = field.parse::<usize>().unwrap();
                    let is_valid = rules.keys().any(|&((start1, end1), (start2, end2))| {
                        (value >= start1 && value <= end1) || (value >= start2 && value <= end2)
                    });
                    if is_valid {
                        Some(value)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            if num_fields == fields.len() {
                fields.iter().enumerate().for_each(|(index, field)| {
                    match ticket_fields.get_mut(index) {
                        Some(tickets) => tickets.push(*field),
                        None => ticket_fields.push(vec![*field]),
                    };
                });
            } else {
            }
        });

    let mut rules_with_fieldindex = HashMap::<String, usize>::new();
    loop {
        ticket_fields
            .iter()
            .enumerate()
            .for_each(|(ticket_field_index, values)| {
                let matching_rules = rules
                    .iter()
                    .filter_map(|(((start1, end1), (start2, end2)), rule_name)| {
                        if rules_with_fieldindex.contains_key(rule_name) {
                            None
                        } else {
                            if values.iter().all(|value| {
                                (value >= start1 && value <= end1)
                                    || (value >= start2 && value <= end2)
                            }) {
                                Some(rule_name.clone())
                            } else {
                                None
                            }
                        }
                    })
                    .collect::<Vec<String>>();

                if matching_rules.len() == 1 {
                    rules_with_fieldindex
                        .entry(matching_rules[0].to_string())
                        .or_insert(ticket_field_index);
                }
            });
        if rules_with_fieldindex.len() == ticket_fields.len() {
            println!("determined rules: {:?}", rules_with_fieldindex);
            break;
        }
    }

    rules_with_fieldindex
        .iter()
        .fold(1, |product, (rule_name, index)| {
            if rule_name.starts_with("departure") {
                product * my_ticket[*index]
            } else {
                product
            }
        })
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
