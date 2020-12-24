use image::ImageBuffer;
use itertools::Itertools;
use std::collections::HashMap;
use std::env;

type Color = (u8, u8, u8);
type Coordinate = (i32, i32);

#[derive(Debug, PartialEq, Eq)]
enum TileSide {
    Black,
    White,
}

#[derive(Debug)]
struct Tile {
    tile_id: usize,
    location: Coordinate,
    path: Vec<Coordinate>,
    color: TileSide,
}

impl Tile {
    fn new(tile_id: usize, line: &str) -> Tile {
        let mut current_tile = (0, 0);
        let path = line
            .chars()
            .batching(|it| match it.next() {
                None => None,
                Some(c) => match c {
                    'w' => {
                        current_tile = (current_tile.0 - 2, current_tile.1);
                        Some(current_tile)
                    }
                    'e' => {
                        current_tile = (current_tile.0 + 2, current_tile.1);
                        Some(current_tile)
                    }
                    's' => match it.next() {
                        Some(c2) => match c2 {
                            'w' => {
                                current_tile = (current_tile.0 - 1, current_tile.1 + 1);
                                Some(current_tile)
                            }
                            'e' => {
                                current_tile = (current_tile.0 + 1, current_tile.1 + 1);
                                Some(current_tile)
                            }
                            _ => None,
                        },
                        None => None,
                    },
                    'n' => match it.next() {
                        Some(c2) => match c2 {
                            'w' => {
                                current_tile = (current_tile.0 - 1, current_tile.1 - 1);
                                Some(current_tile)
                            }
                            'e' => {
                                current_tile = (current_tile.0 + 1, current_tile.1 - 1);
                                Some(current_tile)
                            }
                            _ => None,
                        },
                        None => None,
                    },
                    _ => None,
                },
            })
            .collect::<Vec<Coordinate>>();

        Tile {
            tile_id: tile_id,
            location: *path.last().unwrap(),
            path: path,
            color: TileSide::Black,
        }
    }

    fn flip(&mut self) {
        println!("Flipping: {:?}", self);
        self.color = if self.color == TileSide::Black {
            TileSide::White
        } else {
            TileSide::Black
        };
    }

    fn draw(&self, pixels: &mut Vec<(Coordinate, Color)>) {
        let inner_white_color = (0x96, 0xA7, 0x74);
        let inner_black_color = (0x32, 0x17, 0x04);
        let border_color = (0x33, 0xCC, 0x66);
        let missing_color = (0xB3, 0xCC, 0xA5);

        let scale = 4;
        let tile = [
            "....%...", //
            "...%%%..", //
            "..%%%%%.", //
            ".%%%%%%%", //
            ".%%%%%%%", //
            "..%%%%%.", //
            "...%%%..", //
            "....%...", //
        ];

        tile.iter().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| match c {
                '#' | '%' => {
                    let pixel_color = match c {
                        '#' => border_color,
                        '%' => {
                            if self.color == TileSide::Black {
                                inner_black_color
                            } else {
                                inner_white_color
                            }
                        }
                        _ => missing_color,
                    };
                    pixels.push((
                        (
                            (self.location.0 * scale + x as i32),
                            (self.location.1 * scale + y as i32),
                        ),
                        pixel_color,
                    ));
                }
                _ => (),
            })
        })
    }
}

fn visualize(tiles: &HashMap<Coordinate, Tile>, frame: i32) {
    let max_dimension: u32 = 10;
    let scale: u32 = 8;
    let border: u32 = 2;
    let mut pixels = Vec::<(Coordinate, Color)>::new();

    println!("frame: {}, ", frame);

    tiles.values().for_each(|tile| {
        tile.draw(&mut pixels);
    });

    let x_min = pixels.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = pixels.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = pixels.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = pixels.iter().map(|(pos, _)| pos.1).max().unwrap();
    let x_range = (x_max - x_min) as u32;
    let y_range = (y_max - y_min) as u32;
    let dimensions = (1 + x_range, 1 + y_range);

    let real_size = (
        (dimensions.0 + border * 2) * scale as u32,
        (dimensions.1 + border * 2) * scale as u32,
    );

    let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    for ((raw_x, raw_y), color) in pixels {
        let pixel = image::Rgb([color.0, color.1, color.2]);
        let (x, y) = (raw_x - x_min, raw_y - y_min);
        if x >= 0 && y >= 0 && x < real_size.0 as i32 && y < real_size.1 as i32 {
            for scaled_y in 0..scale {
                for scaled_x in 0..scale {
                    img.put_pixel(
                        ((x as u32 + border) * scale + scaled_x) as u32,
                        ((y as u32 + border) * scale + scaled_y) as u32,
                        pixel,
                    );
                }
            }
        }
    }

    img.save(format!("frames/day24.frame{:05}.png", frame))
        .unwrap();
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut tiles = HashMap::<Coordinate, Tile>::new();
    contents.lines().enumerate().for_each(|(index, line)| {
        let tile = Tile::new(index, line);
        println!("current tile: {:?}", tile);
        tiles
            .entry(tile.location)
            .and_modify(|e| e.flip())
            .or_insert(tile);
    });

    println!("tiles: {:?}", tiles);
    let mut frame = 0;
    visualize(&tiles, frame);
    tiles.values().fold(0, |sum, tile| {
        sum + if tile.color == TileSide::Black { 1 } else { 0 }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    //println!("Part2: {}", solve_part2(args[1].to_string()));
}
