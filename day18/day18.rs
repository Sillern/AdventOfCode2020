use itertools::Itertools;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
enum Operation {
    Addition,
    Multiplication,
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    fn calculate(expression: &mut Vec<String>) -> usize {
        let mut sum = 0;
        let mut operation = Operation::Addition;

        while expression.len() != 0 {
            let token = &expression[0];

            match token.parse::<usize>() {
                Ok(number) => {
                    expression.remove(0);
                    match operation {
                        Operation::Addition => sum += number,
                        Operation::Multiplication => sum *= number,
                    }
                }
                Err(error_message) => {
                    match token.as_ref() {
                        "+" => {
                            expression.remove(0);
                            operation = Operation::Addition;
                        }
                        "*" => {
                            expression.remove(0);
                            operation = Operation::Multiplication;
                        }
                        _ => {
                            if token.starts_with('(') {
                                println!("Token: {}, should recurse", token);
                                expression[0] = token[1..].to_string();
                                let number = calculate(expression);
                                match operation {
                                    Operation::Addition => sum += number,
                                    Operation::Multiplication => sum *= number,
                                }
                            } else {
                                match token.find(')') {
                                    Some(pos) => {
                                        if pos != 0 {
                                            let number = token[..pos].parse::<usize>().unwrap();
                                            match operation {
                                                Operation::Addition => sum += number,
                                                Operation::Multiplication => sum *= number,
                                            }
                                        }
                                        expression[0] = token[pos + 1..].to_string();

                                        return sum;
                                    }
                                    None => {
                                        expression.remove(0);
                                    }
                                }
                            }
                        }
                    };
                }
            };
        }

        sum
    }

    contents.lines().fold(0, |sum, line| {
        let mut expression = line
            .split(' ')
            .map(|token| token.to_string())
            .collect::<Vec<String>>();
        let result = calculate(&mut expression);
        println!("expression: {:#?} = {}", line, result);
        sum + result
    })
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    fn parse_tokens(tokens: &mut Vec<char>) -> usize {
        let mut token_index = 0;
        let mut numbers = Vec::<usize>::new();
        let mut last_operation = Operation::Addition;

        while tokens.len() != 0 {
            let num_tokens = tokens.iter().take_while(|&&c| char::is_numeric(c)).count();

            match tokens
                .iter()
                .take_while(|&&c| char::is_numeric(c))
                .collect::<String>()
                .parse::<usize>()
            {
                Ok(number) => {
                    for _ in 0..num_tokens {
                        tokens.remove(0);
                    }
                    match last_operation {
                        Operation::Addition => match numbers.last_mut() {
                            Some(last_number) => *last_number += number,
                            None => numbers.push(number),
                        },
                        Operation::Multiplication => {
                            numbers.push(number);
                        }
                    }
                    println!("number: {:?}, numbers: {:?}", number, numbers);
                }
                Err(_) => match tokens[0] {
                    '*' => {
                        last_operation = Operation::Multiplication;
                        tokens.remove(0);
                    }
                    '+' => {
                        last_operation = Operation::Addition;
                        tokens.remove(0);
                    }
                    '(' => {
                        tokens.remove(0);
                        println!("paren open, recurse into: {:?}", tokens);
                        let inner_number = parse_tokens(tokens);
                        match last_operation {
                            Operation::Addition => match numbers.last_mut() {
                                Some(last_number) => *last_number += inner_number,
                                None => numbers.push(inner_number),
                            },
                            Operation::Multiplication => {
                                numbers.push(inner_number);
                            }
                        }
                    }
                    ')' => {
                        tokens.remove(0);
                        println!("paren close");
                        break;
                    }
                    _ => {}
                },
            }
        }
        println!("returning product of numbers: {:?}", numbers);
        return numbers.iter().fold(1, |product, number| product * number);
    }

    contents.lines().fold(0, |sum, line| {
        let mut tokens = line.chars().filter(|c| *c != ' ').collect::<Vec<char>>();
        sum + parse_tokens(&mut tokens)
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
