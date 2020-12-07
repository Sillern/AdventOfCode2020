use regex::Regex;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let main_rule = Regex::new(r"(?P<bag_rule>.+)\sbag(s?)\scontain\s(?P<contains>.+).$").unwrap();
    let content_rule =
        Regex::new(r"(?P<number_of_bags>(\d|no)+)\s(?P<bag_rule>.+)\sbag(s?)(\.?)$").unwrap();

    let mut bag_rules = HashMap::<String, Vec<String>>::new();

    contents.lines().for_each(|line| {
        let parsed = main_rule.captures(line).unwrap();

        parsed["contains"].split(", ").for_each(|rule| {
            let bag_rule = parsed["bag_rule"].to_string();
            let parsed_content = content_rule.captures(rule).unwrap();
            let number_of_bags = match parsed_content["number_of_bags"].parse::<i32>() {
                Ok(value) => value,
                Err(_) => 0,
            };

            if number_of_bags > 0 {
                bag_rules
                    .entry(bag_rule)
                    .and_modify(|entry| {
                        entry.push(parsed_content["bag_rule"].to_string());
                    })
                    .or_insert([parsed_content["bag_rule"].to_string()].to_vec());
            }
        });
    });
    fn find_parents(
        bag_rules: &HashMap<String, Vec<String>>,
        child: &str,
        parents: &mut Vec<String>,
    ) {
        bag_rules.iter().for_each(|(bag_id, child_bag_ids)| {
            child_bag_ids.iter().for_each(|child_bag_id| {
                if child_bag_id == child && !parents.contains(bag_id) {
                    parents.push(bag_id.to_string());
                    find_parents(bag_rules, bag_id, parents);
                }
            });
        });
    }

    let mut parents = Vec::<String>::new();
    find_parents(&bag_rules, "shiny gold", &mut parents);
    parents.len() as i32
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let main_rule = Regex::new(r"(?P<bag_rule>.+)\sbag(s?)\scontain\s(?P<contains>.+).$").unwrap();
    let content_rule =
        Regex::new(r"(?P<number_of_bags>(\d|no)+)\s(?P<bag_rule>.+)\sbag(s?)(\.?)$").unwrap();

    let mut bag_rules = HashMap::<String, Vec<(i32, String)>>::new();

    contents.lines().for_each(|line| {
        let parsed = main_rule.captures(line).unwrap();

        parsed["contains"].split(", ").for_each(|rule| {
            let bag_rule = parsed["bag_rule"].to_string();
            let parsed_content = content_rule.captures(rule).unwrap();
            let number_of_bags = match parsed_content["number_of_bags"].parse::<i32>() {
                Ok(value) => value,
                Err(_) => 0,
            };

            bag_rules
                .entry(bag_rule)
                .and_modify(|entry| {
                    entry.push((number_of_bags, parsed_content["bag_rule"].to_string()));
                })
                .or_insert([(number_of_bags, parsed_content["bag_rule"].to_string())].to_vec());
        });
    });
    fn find_children(bag_rules: &HashMap<String, Vec<(i32, String)>>, child: &str) -> i32 {
        match bag_rules.get(child) {
            Some(entry) => entry.iter().fold(1, |sum, (number_of_bags, child_bag_id)| {
                sum + number_of_bags * find_children(bag_rules, child_bag_id)
            }),
            None => 0,
        }
    }

    find_children(&bag_rules, "shiny gold") - 1
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
