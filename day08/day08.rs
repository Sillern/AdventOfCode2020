use regex::Regex;
use std::env;

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc,
    Jmp,
    Unknown,
}

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let instruction_pattern =
        Regex::new(r"(?P<instruction>nop|jmp|acc)\s(?P<value>(\+|\-)\d+)$").unwrap();

    let instructions = contents
        .lines()
        .map(|line| {
            let parsed = instruction_pattern.captures(line).unwrap();
            let instruction = match &parsed["instruction"] {
                "nop" => Instruction::Nop,
                "acc" => Instruction::Acc,
                "jmp" => Instruction::Jmp,
                _ => {
                    println!("unknown instruction");
                    Instruction::Unknown
                }
            };

            let value = match parsed["value"].parse::<i32>() {
                Ok(value) => value,
                Err(_) => 0,
            };

            (instruction, value)
        })
        .collect::<Vec<(Instruction, i32)>>();

    fn process(instructions: Vec<(Instruction, i32)>) -> i32 {
        let mut accumulator = 0;
        let mut pc = 0;

        let mut visited_instructions = Vec::<i32>::new();
        loop {
            if visited_instructions.contains(&pc) {
                return accumulator;
            } else {
                visited_instructions.push(pc);
            }

            if pc < 0 || pc >= instructions.len() as i32 {
                println!("Outside program!");
                return accumulator;
            }

            let (instruction, value) = &instructions[pc as usize];
            match instruction {
                Instruction::Nop => {
                    pc += 1;
                }
                Instruction::Acc => {
                    accumulator += value;
                    pc += 1;
                }
                Instruction::Jmp => {
                    pc += value;
                }
                Instruction::Unknown => {
                    println!("Unknown instruction");
                }
            };
        }
    }

    process(instructions)
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let instruction_pattern =
        Regex::new(r"(?P<instruction>nop|jmp|acc)\s(?P<value>(\+|\-)\d+)$").unwrap();

    let instructions = contents
        .lines()
        .map(|line| {
            let parsed = instruction_pattern.captures(line).unwrap();
            let instruction = match &parsed["instruction"] {
                "nop" => Instruction::Nop,
                "acc" => Instruction::Acc,
                "jmp" => Instruction::Jmp,
                _ => {
                    println!("unknown instruction");
                    Instruction::Unknown
                }
            };

            let value = match parsed["value"].parse::<i32>() {
                Ok(value) => value,
                Err(_) => 0,
            };

            (instruction, value)
        })
        .collect::<Vec<(Instruction, i32)>>();

    fn process(instructions: &Vec<(Instruction, i32)>, fix_instruction: i32) -> (i32, i32) {
        let mut accumulator = 0;
        let mut pc = 0;

        let mut visited_instructions = Vec::<i32>::new();
        loop {
            if visited_instructions.contains(&pc) {
                return (1, accumulator);
            } else {
                visited_instructions.push(pc);
            }

            if pc < 0 || pc >= instructions.len() as i32 {
                return (0, accumulator);
            }

            let (instruction, value) = &instructions[pc as usize];
            let fixed_instruction = if fix_instruction == pc {
                match &instruction {
                    Instruction::Nop => &Instruction::Jmp,
                    Instruction::Jmp => &Instruction::Nop,
                    _ => instruction,
                }
            } else {
                instruction
            };
            match fixed_instruction {
                Instruction::Nop => {
                    pc += 1;
                }
                Instruction::Acc => {
                    accumulator += value;
                    pc += 1;
                }
                Instruction::Jmp => {
                    pc += value;
                }
                Instruction::Unknown => {
                    println!("Unknown instruction");
                }
            };
        }
    }

    for fix_instruction in 0..instructions.len() as i32 {
        let (return_value, value) = process(&instructions, fix_instruction);
        if return_value == 0 {
            return value;
        }
    }
    return -1;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
