use std::env;

fn get_seat_id(boarding_pass: &str) -> i32 {
    boarding_pass
        .chars()
        .take(7)
        .fold((0, 127), |range, c| {
            let middlepoint = range.0 + (range.1 - range.0) / 2;
            match c {
                'B' => (middlepoint + 1, range.1),
                'F' => (range.0, middlepoint),
                _ => {
                    println!("mismatch");
                    (0, 0)
                }
            }
        })
        .0
        * 8
        + boarding_pass
            .chars()
            .skip(7)
            .fold((0, 7), |range, c| {
                let middlepoint = range.0 + (range.1 - range.0) / 2;
                match c {
                    'R' => (middlepoint + 1, range.1),
                    'L' => (range.0, middlepoint),
                    _ => {
                        println!("mismatch");
                        (0, 0)
                    }
                }
            })
            .0
}

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    contents
        .split('\n')
        .fold(0, |highest_seat_id, boarding_pass| {
            let seat_id = get_seat_id(boarding_pass);
            if seat_id > highest_seat_id {
                seat_id
            } else {
                highest_seat_id
            }
        })
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let seats: Vec<i32> = contents
        .split('\n')
        .map(|boarding_pass| get_seat_id(boarding_pass))
        .collect();

    match (0..seats.len() as i32).find(|&seat| {
        !seats.contains(&seat) && seats.contains(&(seat - 1)) && seats.contains(&(seat + 1))
    }) {
        Some(empty_seat) => empty_seat,
        None => {
            println!("Unable to find seat");
            0
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(
        "{:#?}",
        (
            get_seat_id("FBFBBFFRLR"),
            get_seat_id("BFFFBBFRRR"),
            get_seat_id("FFFBBBFRRR"),
            get_seat_id("BBFFBBFRLL")
        )
    );
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
