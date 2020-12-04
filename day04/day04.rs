use itertools::Itertools;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    fn is_valid(passport: &HashMap<String, String>) -> bool {
        let required_fields = vec![
            "byr", // (Birth Year)
            "iyr", // (Issue Year)
            "eyr", // (Expiration Year)
            "hgt", // (Height)
            "hcl", // (Hair Color)
            "ecl", // (Eye Color)
            "pid", // (Passport ID)
        ];
        let optional_fields = vec![
            "cid", // (Country ID)
        ];

        required_fields
            .iter()
            .all(|&field| passport.contains_key(field))
    }
    contents.split("\n\n").fold(0, |sum, passport_info| {
        let mut passport = HashMap::<String, String>::new();
        passport_info
            .split(|c| c == '\n' || c == ' ')
            .for_each(|keyvalue| match keyvalue.split(':').tuples().next() {
                Some((key, value)) => {
                    passport.entry(key.to_string()).or_insert(value.to_string());
                }
                None => (),
            });

        if is_valid(&passport) {
            sum + 1
        } else {
            sum
        }
    })
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    fn is_valid(passport: &HashMap<String, String>) -> bool {
        let required_fields = vec![
            "byr", // (Birth Year)
            "iyr", // (Issue Year)
            "eyr", // (Expiration Year)
            "hgt", // (Height)
            "hcl", // (Hair Color)
            "ecl", // (Eye Color)
            "pid", // (Passport ID)
        ];
        let optional_fields = vec![
            "cid", // (Country ID)
        ];

        if required_fields
            .iter()
            .all(|&field| passport.contains_key(field))
        {
            let valid_birth_year = match passport.get("byr").unwrap().parse::<i32>() {
                Ok(year) => year >= 1920 && year <= 2002,
                Err(_) => false,
            };

            let valid_issue_year = match passport.get("iyr").unwrap().parse::<i32>() {
                Ok(year) => year >= 2010 && year <= 2020,
                Err(_) => false,
            };

            let valid_expiration_year = match passport.get("eyr").unwrap().parse::<i32>() {
                Ok(year) => year >= 2020 && year <= 2030,
                Err(_) => false,
            };

            let valid_height = match passport.get("hgt") {
                Some(value) => match value.get(..value.len() - 2).unwrap().parse::<i32>() {
                    Ok(height) => {
                        if value.ends_with("cm") {
                            height >= 150 && height <= 193
                        } else if value.ends_with("in") {
                            height >= 59 && height <= 76
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                },
                None => false,
            };
            let valid_hair_color = match passport.get("hcl") {
                Some(value) => {
                    value.len() == 7 && value.get(1..).unwrap().chars().all(char::is_alphanumeric)
                }
                None => false,
            };
            let valid_eye_color = match passport.get("ecl") {
                Some(value) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .iter()
                    .any(|color| color == value),
                None => false,
            };
            let valid_passport_id = match passport.get("pid") {
                Some(value) => value.len() == 9 && value.chars().all(char::is_numeric),
                None => false,
            };

            return [
                valid_birth_year,
                valid_issue_year,
                valid_expiration_year,
                valid_height,
                valid_hair_color,
                valid_eye_color,
                valid_passport_id,
            ]
            .iter()
            .all(|&valid| valid);
        } else {
            false
        }
    }

    contents.split("\n\n").fold(0, |sum, passport_info| {
        let mut passport = HashMap::<String, String>::new();
        passport_info
            .split(|c| c == '\n' || c == ' ')
            .for_each(|keyvalue| match keyvalue.split(':').tuples().next() {
                Some((key, value)) => {
                    passport.entry(key.to_string()).or_insert(value.to_string());
                }
                None => (),
            });

        if is_valid(&passport) {
            sum + 1
        } else {
            sum
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
