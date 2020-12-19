use regex::Regex;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct Rule {
    rule_id: i32,
    primary_subrules: Option<Vec<i32>>,
    secondary_subrules: Option<Vec<i32>>,
    rule: Option<char>,
}

impl Rule {
    fn new(rule_line: &str) -> Rule {
        let rule_pattern = Regex::new(r"(?P<rule_id>\d+):\s(((?P<primary_subrules>[\d\s]+)(\s\|\s(?P<secondary_subrules>[\d\s]+))?)|(.(?P<rule>[a|b]).))$").unwrap();

        let parsed = rule_pattern.captures(rule_line).unwrap();
        let rule_id = parsed["rule_id"].parse::<i32>().unwrap();

        Rule {
            rule_id: rule_id,
            primary_subrules: match parsed.name("primary_subrules") {
                Some(subrules) => Some(
                    subrules
                        .as_str()
                        .split(' ')
                        .map(|token| token.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                ),
                None => None,
            },
            secondary_subrules: match parsed.name("secondary_subrules") {
                Some(subrules) => Some(
                    subrules
                        .as_str()
                        .split(' ')
                        .map(|token| token.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                ),
                None => None,
            },
            rule: match parsed.name("rule") {
                Some(rule) => Some(rule.as_str().chars().nth(0).unwrap()),
                None => None,
            },
        }
    }
}

fn matches_rules(
    rules: &HashMap<i32, Rule>,
    rule_index: i32,
    payload: &Vec<char>,
    payload_index: &mut usize,
    indentation: usize,
) -> bool {
    if *payload_index >= payload.len() {
        println!(
            "{:indent$}TOO FAR!, payload_index: {}",
            "",
            payload_index,
            indent = indentation,
        );
        return false;
    }
    let rule = &rules.get(&rule_index).unwrap();

    println!(
        "{:indent$}checking rule {}, payload_index: {}",
        "",
        rule.rule_id,
        payload_index,
        indent = indentation,
    );
    let mut primary_subpayload_index = *payload_index;
    let matches_primary = match &rule.primary_subrules {
        Some(subrules) => {
            println!(
                "{:indent$}Primary, Recursing into rule {:?}, current payload index: {}, remainder: {:?}",
                "",
                subrules,
                *payload_index,
                &payload[*payload_index..],
                indent = indentation
            );
            let matches_subrule = subrules.iter().all(|subrule_index| {
                if matches_rules(
                    rules,
                    *subrule_index,
                    payload,
                    &mut primary_subpayload_index,
                    indentation + 4,
                ) {
                    true
                } else {
                    false
                }
            });

            if matches_subrule {
                println!(
                    "{:indent$}Primary, Recursed out of rule {:?}, current payload index: {}, remainder: {:?}",
                    "",
                    subrules,
                    *payload_index,
                &payload[*payload_index..],
                    indent = indentation
                );
                true
            } else {
                false
            }
        }
        None => false,
    };

    let mut check_secondary = true;
    if rule.rule_id == 8 || rule.rule_id == 11 {
        //check_secondary = !matches_primary;
    }

    let mut secondary_subpayload_index = *payload_index;
    let matches_secondary =
        check_secondary
            && match &rule.secondary_subrules {
                Some(subrules) => {
                    println!(
                        "{:indent$}Secondary, Recursing into rule {:?}, current payload index: {}",
                        "",
                        subrules,
                        *payload_index,
                        indent = indentation
                    );
                    let matches_subrule = subrules.iter().all(|subrule_index| {
                        if matches_rules(
                            rules,
                            *subrule_index,
                            payload,
                            &mut secondary_subpayload_index,
                            indentation + 4,
                        ) {
                            true
                        } else {
                            false
                        }
                    });

                    if matches_subrule {
                        println!(
                        "{:indent$}Secondary, Recursed out of rule {:?}, current payload index: {}",
                        "", subrules, *payload_index, indent=indentation
                    );
                        true
                    } else {
                        false
                    }
                }
                None => false,
            };

    match rule.rule {
        Some(rule_char) => {
            let found_match = if payload.get(*payload_index) == Some(&rule_char) {
                true
            } else {
                false
            };

            *payload_index += 1;
            found_match
        }
        None => {
            println!(
                "{:indent$}found subrule match {} || {}, rule {}",
                "",
                matches_primary,
                matches_secondary,
                rule.rule_id,
                indent = indentation
            );
            if matches_primary && matches_secondary {
                println!(
                    "matches both: {:?} and {:?}",
                    (primary_subpayload_index),
                    (secondary_subpayload_index)
                );
            }
            if matches_primary {
                *payload_index = primary_subpayload_index;
            }
            if matches_secondary {
                *payload_index = secondary_subpayload_index;
            }

            matches_primary || matches_secondary
        }
    }
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut rules = HashMap::<i32, Rule>::new();

    let rule_block = contents.split("\n\n").nth(0).unwrap();
    let payload_block = contents.split("\n\n").nth(1).unwrap();

    rule_block.lines().for_each(|line| {
        let rule = Rule::new(line);
        rules.entry(rule.rule_id).or_insert(rule);
    });

    payload_block.lines().fold(0, |sum, line| {
        let payload = line.chars().collect::<Vec<char>>();
        let mut payload_index = 0;

        if matches_rules(&rules, 0, &payload, &mut payload_index, 0) {
            if payload.len() == payload_index {
                sum + 1
            } else {
                sum
            }
        } else {
            sum
        }
    })
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut rules = HashMap::<i32, Rule>::new();

    let rule_block = contents.split("\n\n").nth(0).unwrap();
    let payload_block = contents.split("\n\n").nth(1).unwrap();

    rule_block.lines().for_each(|line| {
        let mut line_copy = line.clone();
        if line == "8: 42" {
            line_copy = "8: 42 | 42 8";
        }
        if line == "11: 42 31" {
            line_copy = "11: 42 31 | 42 11 31";
        }
        let rule = Rule::new(line_copy);
        rules.entry(rule.rule_id).or_insert(rule);
    });

    payload_block.lines().take(1).fold(0, |sum, line| {
        let payload = line.chars().collect::<Vec<char>>();
        let mut payload_index = 0;

        if matches_rules(&rules, 0, &payload, &mut payload_index, 0) {
            if payload.len() == payload_index {
                sum + 1
            } else {
                sum
            }
        } else {
            sum
        }
    })
}
fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
