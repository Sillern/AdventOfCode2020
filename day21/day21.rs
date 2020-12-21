use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env;

type Range = (usize, usize);

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let rule_pattern = Regex::new(r"(?P<food_words>.+)\s\(contains (?P<allergens>.+)\)$").unwrap();

    let mut allergens_map = HashMap::<String, HashMap<String, i32>>::new();
    let mut allergens_count = HashMap::<String, i32>::new();
    let mut all_ingredients = Vec::<String>::new();

    contents.lines().for_each(|line| {
        let parsed = rule_pattern.captures(line).unwrap();
        let food_words = parsed["food_words"]
            .split(" ")
            .map(|entry| entry.to_string())
            .collect::<Vec<String>>();

        let allergens = parsed["allergens"]
            .split(", ")
            .map(|entry| entry.to_string())
            .collect::<Vec<String>>();

        let num_allergens = allergens.len();

        allergens.iter().for_each(|allergen| {
            println!("{}: in {:?}", allergen, food_words);
            allergens_count
                .entry(allergen.to_string())
                .and_modify(|entry| *entry += 1)
                .or_insert(1);

            allergens_map
                .entry(allergen.to_string())
                .and_modify(|entry| {
                    food_words.iter().for_each(|food_word| {
                        entry
                            .entry(food_word.to_string())
                            .and_modify(|entry| *entry += 1)
                            .or_insert(1);
                    })
                })
                .or_insert({
                    let mut food_word_count = HashMap::<String, i32>::new();
                    food_words.iter().for_each(|food_word| {
                        food_word_count
                            .entry(food_word.to_string())
                            .and_modify(|entry| *entry += 1)
                            .or_insert(1);
                    });
                    food_word_count
                });
        });
        all_ingredients.extend(food_words);
    });

    let mut known_allergens = HashMap::<String, String>::new();

    while known_allergens.len() != allergens_count.len() {
        allergens_map.iter().for_each(|(allergen, counts)| {
            let valid_count = allergens_count[allergen];
            let candidates = counts
                .iter()
                .filter_map(|(food_word, count)| {
                    if *count == valid_count && !known_allergens.contains_key(food_word) {
                        Some(food_word.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>();

            if candidates.len() == 1 {
                println!("Resolved: {} to {:?}", allergen, candidates);
                candidates.iter().for_each(|candidate| {
                    known_allergens
                        .entry(candidate.to_string())
                        .or_insert(allergen.to_string());
                });
            }
        });
    }

    println!("allergens: {:#?}", allergens_map);
    println!("allergens_count: {:#?}", allergens_count);
    println!("known_allergens: {:#?}", known_allergens);

    println!("part2: ");
    for (allergen, ingredient) in known_allergens
        .iter()
        .map(|(ingredient, allergen)| (allergen, ingredient))
        .sorted()
    {
        print!("{},", ingredient);
    }
    println!();

    all_ingredients
        .iter()
        .filter(|ingredient| !known_allergens.contains_key(*ingredient))
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    //println!("Part2: {}", solve_part2(args[1].to_string()));
}
