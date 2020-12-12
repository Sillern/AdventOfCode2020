use itertools::Itertools;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Command {
    North,
    South,
    East,
    West,
    TurnLeft,
    TurnRight,
    Forward,
    Unknown,
}

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let sequence = contents
        .lines()
        .map(|line| {
            (
                match &line[0..1] {
                    "N" => Command::North,
                    "S" => Command::South,
                    "E" => Command::East,
                    "W" => Command::West,
                    "L" => Command::TurnLeft,
                    "R" => Command::TurnRight,
                    "F" => Command::Forward,
                    _ => Command::North,
                },
                line[1..].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(Command, i32)>>();

    let (position, direction) =
        sequence
            .iter()
            .fold(((0, 0), 0), |(position, direction), (command, value)| {
                let mut next_position = position;
                let mut next_direction = direction;
                match command {
                    Command::Forward => match next_direction {
                        0 => next_position.0 += value,
                        90 => next_position.1 += value,
                        180 => next_position.0 -= value,
                        270 => next_position.1 -= value,
                        _ => {
                            println!("Unknown direction: {}", next_direction);
                            ()
                        }
                    },
                    Command::TurnLeft => next_direction = (360 + next_direction - value) % 360,
                    Command::TurnRight => next_direction = (next_direction + value) % 360,
                    Command::North => next_position.1 -= value,
                    Command::South => next_position.1 += value,
                    Command::East => next_position.0 += value,
                    Command::West => next_position.0 -= value,
                    Command::Unknown => (),
                };

                (next_position, next_direction)
            });

    println!("position: {:?}, direction: {:?}", position, direction);
    position.0 + position.1
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let sequence = contents
        .lines()
        .map(|line| {
            (
                match &line[0..1] {
                    "N" => Command::North,
                    "S" => Command::South,
                    "E" => Command::East,
                    "W" => Command::West,
                    "L" => Command::TurnLeft,
                    "R" => Command::TurnRight,
                    "F" => Command::Forward,
                    _ => Command::North,
                },
                line[1..].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(Command, i32)>>();

    let (ship_position, waypoint_position) = sequence.iter().fold(
        ((0, 0), (10, -1)),
        |(ship_position, waypoint_position), (command, value)| {
            let mut next_ship_position = ship_position;
            let mut next_waypoint_position = waypoint_position;
            match command {
                Command::Forward => {
                    next_ship_position.0 += value * next_waypoint_position.0;
                    next_ship_position.1 += value * next_waypoint_position.1;
                }
                Command::TurnLeft => {
                    next_waypoint_position = match (360 - value) % 360 {
                        0 => (waypoint_position.0, waypoint_position.1),
                        90 => (-waypoint_position.1, waypoint_position.0),
                        180 => (-waypoint_position.0, -waypoint_position.1),
                        270 => (waypoint_position.1, -waypoint_position.0),
                        _ => (0, 0),
                    };
                }
                Command::TurnRight => {
                    next_waypoint_position = match value % 360 {
                        0 => (waypoint_position.0, waypoint_position.1),
                        90 => (-waypoint_position.1, waypoint_position.0),
                        180 => (-waypoint_position.0, -waypoint_position.1),
                        270 => (waypoint_position.1, -waypoint_position.0),
                        _ => (0, 0),
                    };
                }
                Command::North => next_waypoint_position.1 -= value,
                Command::South => next_waypoint_position.1 += value,
                Command::East => next_waypoint_position.0 += value,
                Command::West => next_waypoint_position.0 -= value,
                Command::Unknown => {
                    println!("Unknown command: {:?}", (command, value));
                    ()
                }
            };

            (next_ship_position, next_waypoint_position)
        },
    );

    println!(
        "ship_position: {:?}, waypoint {:?}",
        ship_position, waypoint_position
    );
    ship_position.0.abs() + ship_position.1.abs()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
