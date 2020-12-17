use itertools::Itertools;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    type Coordinate = (i32, i32, i32);

    fn get_relevant_cubes(cubes: &HashMap<Coordinate, bool>) -> Vec<(Coordinate, bool, usize)> {
        let directions_to_check = [
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, 1, 1),
            (0, 1, -1),
            (0, -1, 0),
            (0, -1, 1),
            (0, -1, -1),
            (1, 0, 0),
            (1, 0, 1),
            (1, 0, -1),
            (1, 1, 0),
            (1, 1, 1),
            (1, 1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, -1, -1),
            (-1, 0, 0),
            (-1, 0, 1),
            (-1, 0, -1),
            (-1, 1, 0),
            (-1, 1, 1),
            (-1, 1, -1),
            (-1, -1, 0),
            (-1, -1, 1),
            (-1, -1, -1),
        ];
        let mut visited = Vec::<Coordinate>::new();
        let mut relevant_cubes = Vec::<(Coordinate, bool, usize)>::new();

        cubes.iter().for_each(|(coord, active_cube)| {
            directions_to_check.iter().for_each(|direction| {
                let check = (
                    coord.0 + direction.0,
                    coord.1 + direction.1,
                    coord.2 + direction.2,
                );

                let active_adjacent =
                    directions_to_check
                        .iter()
                        .fold(0, |sum, secondary_direction| {
                            if secondary_direction != &(0, 0, 0) {
                                let secondary_check = (
                                    check.0 + secondary_direction.0,
                                    check.1 + secondary_direction.1,
                                    check.2 + secondary_direction.2,
                                );

                                sum + match cubes.get(&secondary_check) {
                                    Some(active) => {
                                        if *active {
                                            1
                                        } else {
                                            0
                                        }
                                    }
                                    None => 0,
                                }
                            } else {
                                sum
                            }
                        });

                match cubes.get(&check) {
                    Some(active) => {
                        let cube = (check, *active, active_adjacent);
                        if !relevant_cubes.contains(&cube) {
                            relevant_cubes.push(cube);
                        }
                    }
                    None => {
                        let cube = (check, false, active_adjacent);
                        visited.push(check);
                        if !relevant_cubes.contains(&cube) {
                            relevant_cubes.push(cube);
                        }
                    }
                };
            });
        });

        relevant_cubes
    }

    fn print_cubes(cubes: &HashMap<Coordinate, bool>) {
        for z in -2..3 {
            println!("z={}", z);
            for y in -5..10 {
                for x in -5..10 {
                    let check = (x, y, z);
                    match cubes.get(&check) {
                        Some(active) => {
                            if *active {
                                print!("#");
                            } else {
                                print!(".");
                            }
                        }
                        None => print!("."),
                    };
                }
                println!();
            }
        }
    }

    let mut cubes = HashMap::<Coordinate, bool>::new();

    contents.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, value)| {
            let coord = (x as i32, y as i32, 0 as i32);
            if value == '#' {
                cubes.entry(coord).or_insert(true);
            }
        });
    });

    println!("initial cubes: {:#?}", cubes);
    //print_cubes(&cubes);
    for generation in 0..6 {
        let mut changes = Vec::<(Coordinate, bool)>::new();

        get_relevant_cubes(&cubes)
            .iter()
            .for_each(|(coord, active, active_adjacent)| {
                if *active && !(*active_adjacent == 2 || *active_adjacent == 3) {
                    changes.push((*coord, false));
                } else if !*active && *active_adjacent == 3 {
                    changes.push((*coord, true));
                }
            });

        //println!("changes: {:?}", changes);
        changes.iter().for_each(|(coord, status)| {
            cubes
                .entry(*coord)
                .and_modify(|e| *e = *status)
                .or_insert(*status);
        });
        //print_cubes(&cubes);
    }

    cubes
        .iter()
        .fold(0, |sum, (_, active)| if *active { sum + 1 } else { sum })
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    type Coordinate = (i32, i32, i32, i32);

    fn get_relevant_cubes(cubes: &HashMap<Coordinate, bool>) -> Vec<(Coordinate, bool, usize)> {
        let directions_to_check = [
            (0, 0, 0, 0),
            (0, 0, 1, 0),
            (0, 0, -1, 0),
            (0, 1, 0, 0),
            (0, 1, 1, 0),
            (0, 1, -1, 0),
            (0, -1, 0, 0),
            (0, -1, 1, 0),
            (0, -1, -1, 0),
            (1, 0, 0, 0),
            (1, 0, 1, 0),
            (1, 0, -1, 0),
            (1, 1, 0, 0),
            (1, 1, 1, 0),
            (1, 1, -1, 0),
            (1, -1, 0, 0),
            (1, -1, 1, 0),
            (1, -1, -1, 0),
            (-1, 0, 0, 0),
            (-1, 0, 1, 0),
            (-1, 0, -1, 0),
            (-1, 1, 0, 0),
            (-1, 1, 1, 0),
            (-1, 1, -1, 0),
            (-1, -1, 0, 0),
            (-1, -1, 1, 0),
            (-1, -1, -1, 0),
            (0, 0, 0, 1),
            (0, 0, 1, 1),
            (0, 0, -1, 1),
            (0, 1, 0, 1),
            (0, 1, 1, 1),
            (0, 1, -1, 1),
            (0, -1, 0, 1),
            (0, -1, 1, 1),
            (0, -1, -1, 1),
            (1, 0, 0, 1),
            (1, 0, 1, 1),
            (1, 0, -1, 1),
            (1, 1, 0, 1),
            (1, 1, 1, 1),
            (1, 1, -1, 1),
            (1, -1, 0, 1),
            (1, -1, 1, 1),
            (1, -1, -1, 1),
            (-1, 0, 0, 1),
            (-1, 0, 1, 1),
            (-1, 0, -1, 1),
            (-1, 1, 0, 1),
            (-1, 1, 1, 1),
            (-1, 1, -1, 1),
            (-1, -1, 0, 1),
            (-1, -1, 1, 1),
            (-1, -1, -1, 1),
            (0, 0, 0, -1),
            (0, 0, 1, -1),
            (0, 0, -1, -1),
            (0, 1, 0, -1),
            (0, 1, 1, -1),
            (0, 1, -1, -1),
            (0, -1, 0, -1),
            (0, -1, 1, -1),
            (0, -1, -1, -1),
            (1, 0, 0, -1),
            (1, 0, 1, -1),
            (1, 0, -1, -1),
            (1, 1, 0, -1),
            (1, 1, 1, -1),
            (1, 1, -1, -1),
            (1, -1, 0, -1),
            (1, -1, 1, -1),
            (1, -1, -1, -1),
            (-1, 0, 0, -1),
            (-1, 0, 1, -1),
            (-1, 0, -1, -1),
            (-1, 1, 0, -1),
            (-1, 1, 1, -1),
            (-1, 1, -1, -1),
            (-1, -1, 0, -1),
            (-1, -1, 1, -1),
            (-1, -1, -1, -1),
        ];

        let mut visited = Vec::<Coordinate>::new();
        let mut relevant_cubes = Vec::<(Coordinate, bool, usize)>::new();

        cubes.iter().for_each(|(coord, active_cube)| {
            directions_to_check.iter().for_each(|direction| {
                let check = (
                    coord.0 + direction.0,
                    coord.1 + direction.1,
                    coord.2 + direction.2,
                    coord.3 + direction.3,
                );

                let active_adjacent =
                    directions_to_check
                        .iter()
                        .fold(0, |sum, secondary_direction| {
                            if secondary_direction != &(0, 0, 0, 0) {
                                let secondary_check = (
                                    check.0 + secondary_direction.0,
                                    check.1 + secondary_direction.1,
                                    check.2 + secondary_direction.2,
                                    check.3 + secondary_direction.3,
                                );

                                sum + match cubes.get(&secondary_check) {
                                    Some(active) => {
                                        if *active {
                                            1
                                        } else {
                                            0
                                        }
                                    }
                                    None => 0,
                                }
                            } else {
                                sum
                            }
                        });

                match cubes.get(&check) {
                    Some(active) => {
                        let cube = (check, *active, active_adjacent);
                        if !relevant_cubes.contains(&cube) {
                            relevant_cubes.push(cube);
                        }
                    }
                    None => {
                        let cube = (check, false, active_adjacent);
                        visited.push(check);
                        if !relevant_cubes.contains(&cube) {
                            relevant_cubes.push(cube);
                        }
                    }
                };
            });
        });

        relevant_cubes
    }

    let mut cubes = HashMap::<Coordinate, bool>::new();

    contents.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, value)| {
            let coord = (x as i32, y as i32, 0 as i32, 0 as i32);
            if value == '#' {
                cubes.entry(coord).or_insert(true);
            }
        });
    });

    for generation in 0..6 {
        let mut changes = Vec::<(Coordinate, bool)>::new();

        get_relevant_cubes(&cubes)
            .iter()
            .for_each(|(coord, active, active_adjacent)| {
                if *active && !(*active_adjacent == 2 || *active_adjacent == 3) {
                    changes.push((*coord, false));
                } else if !*active && *active_adjacent == 3 {
                    changes.push((*coord, true));
                }
            });

        changes.iter().for_each(|(coord, status)| {
            cubes
                .entry(*coord)
                .and_modify(|e| *e = *status)
                .or_insert(*status);
        });
    }

    cubes
        .iter()
        .fold(0, |sum, (_, active)| if *active { sum + 1 } else { sum })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
