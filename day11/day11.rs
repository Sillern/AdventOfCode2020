use itertools::Itertools;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq)]
enum SeatingStatus {
    Floor,
    EmptySeat,
    OccupiedSeat,
    Unknown,
}

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut seating_map = HashMap::<Coordinate, SeatingStatus>::new();

    let mut dimensions: Coordinate = (0, 0);
    contents.lines().enumerate().for_each(|(y, row)| {
        dimensions.1 = y as i32 + 1;
        row.chars().enumerate().for_each(|(x, value)| {
            if (x as i32 + 1) > dimensions.0 {
                dimensions.0 = x as i32 + 1;
            }
            let coord = (x as i32, y as i32);
            seating_map.entry(coord).or_insert(match value {
                '.' => SeatingStatus::Floor,
                '#' => SeatingStatus::OccupiedSeat,
                'L' => SeatingStatus::EmptySeat,
                _ => SeatingStatus::Unknown,
            });
        });
    });

    fn occupy_seat(
        seating_map: &HashMap<Coordinate, SeatingStatus>,
        coord: &Coordinate,
    ) -> SeatingStatus {
        let coords_to_check = [
            (-1, -1),
            (-1, 1),
            (-1, 0),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 1),
            (1, 0),
        ];

        let occupied_adjacent_seats = coords_to_check.iter().fold(0, |sum, relative_coord| {
            let check = (coord.0 + relative_coord.0, coord.1 + relative_coord.1);
            match seating_map.get(&check) {
                Some(valid_seating) => {
                    sum + match valid_seating {
                        SeatingStatus::OccupiedSeat => 1,
                        _ => 0,
                    }
                }
                None => sum,
            }
        });

        match seating_map.get(&coord) {
            Some(valid_seating) => match valid_seating {
                SeatingStatus::EmptySeat => {
                    if occupied_adjacent_seats == 0 {
                        SeatingStatus::OccupiedSeat
                    } else {
                        *valid_seating
                    }
                }
                SeatingStatus::OccupiedSeat => {
                    if occupied_adjacent_seats >= 4 {
                        SeatingStatus::EmptySeat
                    } else {
                        *valid_seating
                    }
                }
                _ => *valid_seating,
            },
            None => {
                println!("Unknown seating combination at: {:?}", coord);
                SeatingStatus::Unknown
            }
        }
    }

    fn run_generation(seating_map: &mut HashMap<Coordinate, SeatingStatus>) -> usize {
        let seating_changes = seating_map
            .iter()
            .filter_map(|(coord, seat_status)| {
                let next_status = occupy_seat(&seating_map, coord);

                if *seat_status == next_status {
                    None
                } else {
                    Some((*coord, next_status))
                }
            })
            .collect::<Vec<(Coordinate, SeatingStatus)>>();

        seating_changes.iter().for_each(|(coord, status)| {
            seating_map.entry(*coord).and_modify(|e| *e = *status);
        });

        seating_changes.len()
    }

    loop {
        let seat_changes = run_generation(&mut seating_map);
        println!("seat_changes: {}", seat_changes);
        if seat_changes == 0 {
            break;
        }
    }

    seating_map.iter().fold(0, |sum, (_, seat_status)| {
        if *seat_status == SeatingStatus::OccupiedSeat {
            sum + 1
        } else {
            sum
        }
    })
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut seating_map = HashMap::<Coordinate, SeatingStatus>::new();

    let mut dimensions: Coordinate = (0, 0);
    contents.lines().enumerate().for_each(|(y, row)| {
        dimensions.1 = y as i32 + 1;
        row.chars().enumerate().for_each(|(x, value)| {
            if (x as i32 + 1) > dimensions.0 {
                dimensions.0 = x as i32 + 1;
            }
            let coord = (x as i32, y as i32);
            seating_map.entry(coord).or_insert(match value {
                '.' => SeatingStatus::Floor,
                '#' => SeatingStatus::OccupiedSeat,
                'L' => SeatingStatus::EmptySeat,
                _ => SeatingStatus::Unknown,
            });
        });
    });

    fn occupy_seat(
        seating_map: &HashMap<Coordinate, SeatingStatus>,
        coord: &Coordinate,
    ) -> SeatingStatus {
        let directions_to_check = [
            (-1, -1),
            (-1, 1),
            (-1, 0),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 1),
            (1, 0),
        ];

        let occupied_adjacent_seats = directions_to_check.iter().fold(0, |sum, direction| {
            let mut check = *coord;
            let mut is_direction_occupied = false;
            loop {
                check.0 += direction.0;
                check.1 += direction.1;

                match seating_map.get(&check) {
                    Some(valid_seating) => match valid_seating {
                        SeatingStatus::OccupiedSeat => {
                            is_direction_occupied = true;
                            break;
                        }
                        SeatingStatus::EmptySeat => {
                            break;
                        }
                        _ => (),
                    },
                    None => break,
                };
            }
            if is_direction_occupied {
                sum + 1
            } else {
                sum
            }
        });

        match seating_map.get(&coord) {
            Some(valid_seating) => match valid_seating {
                SeatingStatus::EmptySeat => {
                    if occupied_adjacent_seats == 0 {
                        SeatingStatus::OccupiedSeat
                    } else {
                        *valid_seating
                    }
                }
                SeatingStatus::OccupiedSeat => {
                    if occupied_adjacent_seats >= 5 {
                        SeatingStatus::EmptySeat
                    } else {
                        *valid_seating
                    }
                }
                _ => *valid_seating,
            },
            None => {
                println!("Unknown seating combination at: {:?}", coord);
                SeatingStatus::Unknown
            }
        }
    }

    fn run_generation(seating_map: &mut HashMap<Coordinate, SeatingStatus>) -> usize {
        let seating_changes = seating_map
            .iter()
            .filter_map(|(coord, seat_status)| {
                let next_status = occupy_seat(&seating_map, coord);

                if *seat_status == next_status {
                    None
                } else {
                    Some((*coord, next_status))
                }
            })
            .collect::<Vec<(Coordinate, SeatingStatus)>>();

        seating_changes.iter().for_each(|(coord, status)| {
            seating_map.entry(*coord).and_modify(|e| *e = *status);
        });

        seating_changes.len()
    }

    loop {
        let seat_changes = run_generation(&mut seating_map);
        println!("seat_changes: {}", seat_changes);
        if seat_changes == 0 {
            break;
        }
    }

    seating_map.iter().fold(0, |sum, (_, seat_status)| {
        if *seat_status == SeatingStatus::OccupiedSeat {
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
