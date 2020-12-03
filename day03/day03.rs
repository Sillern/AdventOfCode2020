use image::ImageBuffer;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
struct HillPath {
    current: Coordinate,
    dimensions: Coordinate,
    step: Coordinate,
    repeats: u32,
}

impl HillPath {
    fn new(start_pos: Coordinate, dimensions: Coordinate, step: Coordinate) -> HillPath {
        HillPath {
            current: start_pos,
            dimensions,
            step,
            repeats: 0,
        }
    }
}

impl Iterator for HillPath {
    type Item = (Coordinate, u32);
    fn next(&mut self) -> Option<(Coordinate, u32)> {
        if self.current.1 >= self.dimensions.1 {
            None
        } else {
            if (self.current.0 + self.step.0) >= self.dimensions.0 {
                self.repeats += 1;
            }

            self.current.0 = (self.current.0 + self.step.0) % self.dimensions.0;
            self.current.1 += self.step.1;

            if self.current.1 >= self.dimensions.1 {
                None
            } else {
                Some((self.current, self.repeats))
            }
        }
    }
}

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let mut tree_map = HashMap::<Coordinate, u32>::new();

    let mut dimensions: Coordinate = (0, 0);
    contents.lines().enumerate().for_each(|(y, row)| {
        dimensions.1 = y as i32 + 1;
        row.chars().enumerate().for_each(|(x, value)| {
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

    HillPath::new(start_coord, dimensions, (3, 1)).fold(0, |sum, (pos, _)| {
        if tree_map.contains_key(&pos) {
            sum + 1
        } else {
            sum
        }
    })
}

fn solve_part2(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let mut tree_map = HashMap::<Coordinate, u32>::new();

    let mut dimensions: Coordinate = (0, 0);
    contents.lines().enumerate().for_each(|(y, row)| {
        dimensions.1 = y as i32 + 1;
        row.chars().enumerate().for_each(|(x, value)| {
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
            * HillPath::new(start_coord, dimensions, *steps).fold(0, |sum, (pos, _)| {
                if tree_map.contains_key(&pos) {
                    sum + 1
                } else {
                    sum
                }
            })
    })
}

type Color = (u8, u8, u8);

fn draw_tree(pixels: &mut Vec<(i32, i32, Color)>, center: (i32, i32), scale: i32) {
    fn draw_tree_inner(pixels: &mut Vec<(i32, i32, Color)>, top_left: (i32, i32)) {
        let tree = vec![
            "00011000", "00111100", "01011010", "01111110", "10111101", "11111111", "00022000",
            "00022000",
        ];

        tree.iter().enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, value)| {
                let color = match value {
                    '1' => (0x33, 0xCC, 0x44),
                    '2' => (0xAA, 0x66, 0xCC),
                    _ => (0xCC, 0x66, 0x44),
                };
                if value != '0' {
                    pixels.push(((top_left.0 + x as i32), (top_left.1 + y as i32), color));
                }
            });
        });
    }

    let scaled_top_left = (center.0 * scale - scale / 2, center.1 * scale - scale / 2);
    draw_tree_inner(pixels, scaled_top_left);
}

fn draw_fallen_tree(pixels: &mut Vec<(i32, i32, Color)>, center: (i32, i32), scale: i32) {
    fn draw_tree_inner(pixels: &mut Vec<(i32, i32, Color)>, top_left: (i32, i32)) {
        let tree = vec![
            "00000000", "00000000", "00000000", "00010100", "00111100", "11112122", "01111222",
            "00000000",
        ];

        tree.iter().enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, value)| {
                let color = match value {
                    '1' => (0x33, 0xCC, 0x44),
                    '2' => (0xAA, 0x66, 0xCC),
                    _ => (0xCC, 0x66, 0x44),
                };
                if value != '0' {
                    pixels.push(((top_left.0 + x as i32), (top_left.1 + y as i32), color));
                }
            });
        });
    }

    let scaled_top_left = (center.0 * scale - scale / 2, center.1 * scale - scale / 2);
    draw_tree_inner(pixels, scaled_top_left);
}

struct LineOfSight {
    next: Coordinate,
    end: Coordinate,
    x_diff: i32,
    y_diff: i32,
    x_sign: i32,
    y_sign: i32,
    error_value: i32,
}
impl LineOfSight {
    fn new(start_pos: Coordinate, end_pos: Coordinate) -> LineOfSight {
        let x_diff = (end_pos.0 - start_pos.0).abs();
        let y_diff = -(end_pos.1 - start_pos.1).abs();

        let x_sign = if start_pos.0 < end_pos.0 { 1 } else { -1 };
        let y_sign = if start_pos.1 < end_pos.1 { 1 } else { -1 };

        LineOfSight {
            next: start_pos,
            end: end_pos,
            x_diff,
            y_diff,
            x_sign,
            y_sign,
            error_value: x_diff + y_diff,
        }
    }
}

impl Iterator for LineOfSight {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Coordinate> {
        if self.next.0 == self.end.0 && self.next.1 == self.end.1 {
            None
        } else {
            let current = self.next;
            let double_error_value = 2 * self.error_value;

            if double_error_value >= self.y_diff {
                self.error_value += self.y_diff;
                self.next.0 += self.x_sign;
            }

            if double_error_value <= self.x_diff {
                self.error_value += self.x_diff;
                self.next.1 += self.y_sign;
            }

            if current.0 >= self.end.0 && current.1 >= self.end.1 {
                None
            } else {
                Some(current)
            }
        }
    }
}

fn draw_forest(inputfile: String) {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let mut tree_map = HashMap::<Coordinate, u32>::new();

    contents.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, value)| {
            if value != '.' {
                let coord = (x as i32, y as i32);
                tree_map.entry(coord).or_insert(1);
            }
        });
    });

    let start_coord = (0 as i32, 0 as i32);

    let x_min = tree_map.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = tree_map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = tree_map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = tree_map.iter().map(|(pos, _)| pos.1).max().unwrap();
    let x_range = 1 + (x_max - x_min) as u32;
    let y_range = 1 + (y_max - y_min) as u32;
    let dimensions: Coordinate = (x_range as i32, y_range as i32);

    let steps = (3, 1);
    let repeating_patterns =
        1 + HillPath::new(start_coord, dimensions, steps).fold(0, |max_repeats, (_, repeats)| {
            if repeats > max_repeats {
                repeats
            } else {
                max_repeats
            }
        });

    let scale = 8;
    let border = 2;
    let size = (x_range * repeating_patterns, y_range);
    let real_size = (
        ((size.0 + border * 2) * scale as u32),
        ((size.1 + border * 2) * scale as u32),
    );

    for iteration in 0..1 {
        let mut pixels = Vec::<(i32, i32, Color)>::new();
        for (tree, exists) in tree_map.iter() {
            if *exists == 0 {
                continue;
            }
            for repeat in 0..repeating_patterns {
                let tree_coord = (tree.0 + (repeat * x_range) as i32, tree.1);
                if HillPath::new(start_coord, dimensions, steps)
                    .any(|(path_pos, repeats)| path_pos == *tree && repeats == repeat)
                {
                    draw_fallen_tree(&mut pixels, tree_coord, scale);
                } else {
                    draw_tree(&mut pixels, tree_coord, scale);
                }
            }
        }

        let start_coord = (0 as i32, 0 as i32);

        let mut previous_pos = start_coord;
        let mut previous_repeat_offset = 0;
        let path_color = (0xCC, 0xBB, 0xCC);

        // take(iteration).
        HillPath::new(start_coord, dimensions, steps).for_each(|(next_pos, repeats)| {
            let next_repeat_offset = (repeats * x_range) as i32;
            let scaled_previous_pos = (
                (previous_pos.0 + previous_repeat_offset) * scale,
                previous_pos.1 * scale,
            );
            let scaled_next_pos = (
                (next_pos.0 + next_repeat_offset) * scale,
                next_pos.1 * scale,
            );

            previous_pos = next_pos;
            previous_repeat_offset = next_repeat_offset;

            LineOfSight::new(scaled_previous_pos, scaled_next_pos).for_each(|pixel_pos| {
                pixels.push((pixel_pos.0, pixel_pos.1, path_color));
            });
        });

        let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
            image::Rgb([255, 255, 255])
        });

        for pos in pixels {
            let x = pos.0 + (border as i32 * scale);
            let y = pos.1 + (border as i32 * scale);
            let color = pos.2;

            let pixel = image::Rgb([color.0, color.1, color.2]);
            if x >= 0 && y >= 0 && x < real_size.0 as i32 && y < real_size.1 as i32 {
                img.put_pixel(x as u32, y as u32, pixel);
            }
        }

        img.save(format!("frames/day03.frame{:05}.png", iteration))
            .unwrap();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));

    draw_forest(args[1].to_string());
}
