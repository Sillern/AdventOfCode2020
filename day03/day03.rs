use image::ImageBuffer;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
struct HillPath {
    current: Coordinate,
    next: Coordinate,
    dimensions: Coordinate,
    step: Coordinate,
}

impl HillPath {
    fn new(start_pos: Coordinate, dimensions: Coordinate, step: Coordinate) -> HillPath {
        HillPath {
            current: start_pos,
            next: start_pos,
            dimensions,
            step,
        }
    }
}

impl Iterator for HillPath {
    type Item = (Coordinate, u32);
    fn next(&mut self) -> Option<(Coordinate, u32)> {
        if self.current.1 >= self.dimensions.1 {
            None
        } else {
            self.current = (self.next.0 % self.dimensions.0, self.next.1);
            let repeats = (self.next.0 / self.dimensions.0) as u32;

            self.next.0 += self.step.0;
            self.next.1 += self.step.1;

            if self.current.1 >= self.dimensions.1 {
                None
            } else {
                Some((self.current, repeats))
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

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    slopes.iter().fold(1, |product, slope| {
        product
            * HillPath::new(start_coord, dimensions, *slope).fold(0, |sum, (pos, _)| {
                if tree_map.contains_key(&pos) {
                    sum + 1
                } else {
                    sum
                }
            })
    })
}

type Color = (u8, u8, u8);

fn draw_symbol(pixels: &mut Vec<(i32, i32, Color)>, top_left: (i32, i32), pattern: &str) {
    pattern.split('\n').enumerate().for_each(|(y, row)| {
        row.chars()
            .map(|value| value.to_digit(10).unwrap() as i32)
            .enumerate()
            .for_each(|(x, value)| {
                let color = match value {
                    1 => (0x33, 0xCC, 0x44),
                    2 => (0x8A, 0x26, 0x3C),
                    3 => (0xAA, 0x36, 0x3C),
                    _ => (0xCC, 0x66, 0x44),
                };

                if value != 0 {
                    pixels.push(((top_left.0 + x as i32), (top_left.1 + y as i32), color));
                }
            });
    });
}

fn draw_scaled_sled(pixels: &mut Vec<(i32, i32, Color)>, center: (i32, i32)) {
    let symbol = "0000000000000000
0000000000033000
3300000000003330
2300000000000033
3333223333330033
3233332333320330
0022000033000330
0022000022003300
3333333333333000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000";

    let top_left = (center.0 - 8, center.1 - 8);
    draw_symbol(pixels, top_left, symbol);
}

fn draw_tree(pixels: &mut Vec<(i32, i32, Color)>, center: (i32, i32), scale: i32) {
    let symbol = "0000000110000000
0000001110000000
0000001111000000
0000011111100000
0001111111110000
0010011111111100
0000111111100000
0001111111110000
0011111111111000
0110111111011100
0000111111100000
0001111111111000
0011111121111100
0110001221000110
1000002222000001
0000002222000000";
    let top_left = (center.0 * scale - scale / 2, center.1 * scale - scale / 2);

    draw_symbol(pixels, top_left, symbol);
}

fn draw_fallen_tree(pixels: &mut Vec<(i32, i32, Color)>, center: (i32, i32), scale: i32) {
    let symbol = "0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000000000000000
0000100100100000
0001101101100022
0011111111112222
0111111111122222
1111111111112200
0111111111111000";
    let top_left = (center.0 * scale - scale / 2, center.1 * scale - scale / 2);

    draw_symbol(pixels, top_left, symbol);
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

    let x_min = tree_map.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = tree_map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = tree_map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = tree_map.iter().map(|(pos, _)| pos.1).max().unwrap();
    let x_range = (x_max - x_min) as u32;
    let y_range = (y_max - y_min) as u32;
    let dimensions: Coordinate = (1 + x_range as i32, 1 + y_range as i32);

    let slope = (3, 1);
    let scale: i32 = 16;
    let border = 2;
    let viewport = (15, 12);
    let real_size = (
        ((viewport.0 + border * 2) * scale as u32),
        ((viewport.1 + border * 2) * scale as u32),
    );

    let start_pos = (0, 0);
    let iframes = scale;

    let viewport_range_x = (
        -(viewport.0 as i32 / 2 + border as i32),
        viewport.0 as i32 + (slope.0 + viewport.0 as i32 + border as i32),
    );
    let viewport_range_y = (
        -(viewport.1 as i32 / 2 + border as i32),
        viewport.1 as i32 + (slope.1 + viewport.1 as i32 + border as i32),
    );

    let mut previous_trees = Vec::<(i32, i32)>::new();

    HillPath::new(start_pos, dimensions, slope)
        .enumerate()
        .for_each(|(pframe, (block_offset, repeat_index))| {
            for iframe in 0..iframes {
                let frame = pframe as i32 * iframes + iframe as i32;
                println!("pframe {}, iframe {}, frame {}", pframe, iframe, frame);

                let mut pixels = Vec::<(i32, i32, Color)>::new();

                let interpolated_offset = (
                    scale * (block_offset.0 + dimensions.0 * repeat_index as i32)
                        + (iframe * scale * slope.0) / iframes,
                    scale * block_offset.1 + (iframe * scale * slope.1) / iframes,
                );
                let path_block_pos = (
                    block_offset.0 + (dimensions.0 * repeat_index as i32),
                    block_offset.1,
                );

                for y in viewport_range_y.0..viewport_range_y.1 {
                    for x in viewport_range_x.0..viewport_range_x.1 {
                        let block_pos = (x + path_block_pos.0, y + path_block_pos.1);
                        let tree_pos = (
                            block_pos.0.abs() % (dimensions.0) as i32,
                            block_pos.1.abs() % (dimensions.1) as i32,
                        );

                        if tree_map.contains_key(&tree_pos) {
                            if block_pos == path_block_pos {
                                previous_trees.push(block_pos);
                            }

                            if previous_trees.contains(&block_pos) {
                                draw_fallen_tree(&mut pixels, block_pos, scale);
                            } else {
                                draw_tree(&mut pixels, block_pos, scale);
                            }
                        }
                    }
                }

                draw_scaled_sled(&mut pixels, interpolated_offset);

                let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
                    image::Rgb([255, 255, 255])
                });

                for pos in pixels {
                    let x = (pos.0 - interpolated_offset.0)
                        + ((viewport.0 as i32 / 2 + border as i32) * scale);
                    let y = (pos.1 - interpolated_offset.1)
                        + ((viewport.1 as i32 / 2 + border as i32) * scale);
                    let color = pos.2;

                    let pixel = image::Rgb([color.0, color.1, color.2]);
                    if x >= 0 && y >= 0 && x < real_size.0 as i32 && y < real_size.1 as i32 {
                        img.put_pixel(x as u32, y as u32, pixel);
                    }
                }

                img.save(format!("frames/day03.frame{:05}.png", frame))
                    .unwrap();
            }
        });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));

    draw_forest(args[1].to_string());
}
