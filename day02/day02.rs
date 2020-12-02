use regex::Regex;
use std::env;

fn solve_part1(inputfile: String) -> i32 {
    let contents = std::fs::read_to_string(inputfile.to_string())
        .expect("Something went wrong reading the file");

    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>\w):\s(?P<password>\w+)").unwrap();

    return contents.lines().fold(0, |valid_passwords, line| {
        let parsed = re.captures(line).unwrap();
        let min = parsed["min"].parse::<i32>().unwrap();
        let max = parsed["max"].parse::<i32>().unwrap();
        let letter = parsed["letter"].chars().next().unwrap();
        let filtered_password = parsed["password"].chars().fold(0, |sum, c| {
            sum + match letter == c {
                true => 1,
                false => 0,
            }
        });

        valid_passwords
            + match filtered_password >= min && filtered_password <= max {
                true => 1,
                false => 0,
            }
    });
}

fn solve_part2(inputfile: String) -> i32 {
    let contents = std::fs::read_to_string(inputfile.to_string())
        .expect("Something went wrong reading the file");

    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>\w):\s(?P<password>\w+)").unwrap();

    return contents.lines().fold(0, |valid_passwords, line| {
        let parsed = re.captures(line).unwrap();
        let min = parsed["min"].parse::<usize>().unwrap();
        let max = parsed["max"].parse::<usize>().unwrap();
        let letter = parsed["letter"].chars().next().unwrap();
        let contains_first = parsed["password"].chars().nth(min - 1usize).unwrap() == letter;
        let contains_second = parsed["password"].chars().nth(max - 1usize).unwrap() == letter;

        valid_passwords
            + match (contains_first || contains_second) && (contains_first != contains_second) {
                true => 1,
                false => 0,
            }
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
