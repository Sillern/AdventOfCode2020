use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
struct HillPath {
    current: Coordinate,
    dimensions: Coordinate,
    step: Coordinate,
}

impl HillPath {
    fn new(start_pos: Coordinate, dimensions: Coordinate, step: Coordinate) -> HillPath {
        HillPath {
            current: start_pos,
            dimensions,
            step,
        }
    }
}

impl Iterator for HillPath {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Coordinate> {
        if self.current.1 >= self.dimensions.1 {
            None
        } else {
            self.current.0 = (self.current.0 + self.step.0) % self.dimensions.0;
            self.current.1 += self.step.1;

            if self.current.1 >= self.dimensions.1 {
                None
            } else {
                Some(self.current)
            }
        }
    }
}

fn solve_part1(inputfile: String) -> i32 {
    let contents = std::fs::read_to_string(inputfile.to_string())
        .expect("Something went wrong reading the file");
    let mut tree_map = HashMap::<Coordinate, u32>::new();

    let mut dimensions: Coordinate = (0, 0);
    contents
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(y, row)| {
            dimensions.1 = y as i32 + 1;
            row.chars().into_iter().enumerate().for_each(|(x, value)| {
                if (x as i32 + 1) > dimensions.0 {
                    dimensions.0 = x as i32 + 1;
                }
                if value != '.' {
                    let coord = (x as i32, y as i32);
                    tree_map.entry(coord).or_insert(1);
                }
            });
        });

    let start_coord = (0 as i32, 0 as i32);

    HillPath::new(start_coord, dimensions, (3, 1)).fold(0, |sum, pos| {
        if tree_map.contains_key(&pos) {
            sum + 1
        } else {
            sum
        }
    })
}

fn solve_part2(inputfile: String) -> i32 {
    let contents = std::fs::read_to_string(inputfile.to_string())
        .expect("Something went wrong reading the file");
    let mut tree_map = HashMap::<Coordinate, u32>::new();

    let mut dimensions: Coordinate = (0, 0);
    contents
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(y, row)| {
            dimensions.1 = y as i32 + 1;
            row.chars().into_iter().enumerate().for_each(|(x, value)| {
                if (x as i32 + 1) > dimensions.0 {
                    dimensions.0 = x as i32 + 1;
                }
                if value != '.' {
                    let coord = (x as i32, y as i32);
                    tree_map.entry(coord).or_insert(1);
                }
            });
        });

    let start_coord = (0 as i32, 0 as i32);

    let steps = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    steps.iter().fold(1, |product, steps| {
        product
            * HillPath::new(start_coord, dimensions, *steps).fold(0, |sum, pos| {
                if tree_map.contains_key(&pos) {
                    sum + 1
                } else {
                    sum
                }
            })
    })
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
