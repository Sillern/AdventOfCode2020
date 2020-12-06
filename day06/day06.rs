use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents.split("\n\n").fold(0, |sum, questions_group| {
        let mut questions = Vec::<char>::new();
        sum + questions_group.split_whitespace().fold(0, |sum, keyvalue| {
            sum + keyvalue.chars().fold(0, |sum, c| {
                sum + if !questions.contains(&c) {
                    questions.push(c);
                    1
                } else {
                    0
                }
            })
        })
    })
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents.split("\n\n").fold(0, |sum, questions_group| {
        let mut questions = HashMap::<char, i32>::new();
        let num_people = questions_group.split_whitespace().fold(0, |sum, answers| {
            answers.chars().for_each(|c| {
                questions.entry(c).and_modify(|e| *e += 1).or_insert(1);
            });
            sum + 1
        });

        let valid_questions = questions.values().fold(0, |sum, &num_answers| {
            if num_answers == num_people {
                sum + 1
            } else {
                sum
            }
        });
        sum + valid_questions
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
