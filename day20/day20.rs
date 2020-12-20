use image::ImageBuffer;
use regex::Regex;
use std::collections::HashMap;
use std::env;

type Color = (u8, u8, u8);
type Coordinate = (i32, i32);

#[derive(Debug)]
struct Tile {
    tile_id: i32,
    location: Coordinate,
    rotation: i32,
    pixels: Vec<Coordinate>,
    borders: Vec<i32>,
}

impl Tile {
    fn new(block: &str) -> Tile {
        let tile_id_pattern = Regex::new(r"Tile (?P<tile_id>\d+):$").unwrap();

        let parsed = tile_id_pattern
            .captures(block.lines().nth(0).unwrap())
            .unwrap();
        let tile_id = parsed["tile_id"].parse::<i32>().unwrap();

        let mut pixels = Vec::<Coordinate>::new();

        block.lines().skip(1).enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, value)| {
                let coord = (x as i32, y as i32);
                if value != '.' {
                    pixels.push(coord);
                }
            });
        });

        Tile {
            tile_id: tile_id,
            location: (-1, -1),
            rotation: 0,
            borders: Tile::get_borders(&pixels),
            pixels: pixels,
        }
    }

    fn get_borders(pixels: &Vec<Coordinate>) -> Vec<i32> {
        let mut borders = Vec::<i32>::new();

        let max_dimension = 10;
        let max_index = max_dimension - 1;

        // top row
        let mut bitpattern = 0;
        let mut bitpattern_reversed = 0;
        for bit in 0..max_dimension {
            if pixels.contains(&(bit, 0)) {
                bitpattern_reversed |= 1 << bit;
                bitpattern |= 1 << (max_index - bit);
            }
        }

        borders.push(bitpattern);
        borders.push(bitpattern_reversed);

        // bottom row
        bitpattern = 0;
        bitpattern_reversed = 0;
        for bit in 0..max_dimension {
            if pixels.contains(&(bit, max_index)) {
                bitpattern_reversed |= 1 << bit;
                bitpattern |= 1 << (max_index - bit);
            }
        }

        borders.push(bitpattern);
        borders.push(bitpattern_reversed);

        // left edge
        bitpattern = 0;
        bitpattern_reversed = 0;
        for bit in 0..max_dimension {
            if pixels.contains(&(0, bit)) {
                bitpattern_reversed |= 1 << bit;
                bitpattern |= 1 << (max_index - bit);
            }
        }

        borders.push(bitpattern);
        borders.push(bitpattern_reversed);

        // right edge
        bitpattern = 0;
        bitpattern_reversed = 0;
        for bit in 0..max_dimension {
            if pixels.contains(&(max_index, bit)) {
                bitpattern_reversed |= 1 << bit;
                bitpattern |= 1 << (max_index - bit);
            }
        }

        borders.push(bitpattern);
        borders.push(bitpattern_reversed);

        borders
    }

    fn draw(&self, pixels: &mut Vec<(Coordinate, Color)>, top_left: (i32, i32)) {
        let max_dimension = 10;
        let color = (0x56, 0x67, 0x44);
        let border_color = (0x33, 0xCC, 0x66);
        for y in 0..max_dimension {
            for x in 0..max_dimension {
                if self.pixels.contains(&(x, y)) {
                    let pixel_color =
                        if x == 0 || y == 0 || x == (max_dimension - 1) || y == (max_dimension - 1)
                        {
                            border_color
                        } else {
                            color
                        };
                    pixels.push((
                        ((top_left.0 + x as i32), (top_left.1 + y as i32)),
                        pixel_color,
                    ));
                }
            }
        }
    }

    fn find_matching_border(&self, tiles: &Tile) -> bool {
        let mut found_match = false;
        self.borders.iter().for_each(|border| {
            let matching_borders = self
                .borders
                .iter()
                .filter(|other_border| border == *other_border)
                .count();

            if matching_borders > 0 {
                println!("Found matching border");
                found_match = true
            }
        });

        found_match
    }
    fn find_matching_borders(&self, tiles: &Vec<Tile>) -> i32 {
        tiles.iter().for_each(|tile| {
            tile.find_matching_border(self);
        });
        0
    }
}

fn solve_part1(inputfile: String) -> i32 {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let max_dimension: u32 = 10;
    let tiles = contents
        .split("\n\n")
        .map(|tile_block| {
            let tile = Tile::new(tile_block);
            println!("tile: {:?}", tile);
            tile
        })
        .collect::<Vec<Tile>>();

    tiles.iter().take(1).for_each(|tile| {
        tile.find_matching_borders(&tiles);
    });

    visualize(&tiles);
    0
}

fn visualize(tiles: &Vec<Tile>) {
    let max_dimension: u32 = 10;
    let num_blocks = (tiles.len() as f32).sqrt() as u32;
    let scale: u32 = 8;
    let border: u32 = 2;
    let block_size = max_dimension + border;
    let frame = 0;
    let mut pixels = Vec::<(Coordinate, Color)>::new();

    tiles.iter().enumerate().for_each(|(index, tile)| {
        let x = index as u32 % num_blocks;
        let y = index as u32 / num_blocks;
        tile.draw(
            &mut pixels,
            ((x * block_size) as i32, (y * block_size) as i32),
        );
    });

    let real_size = (
        (num_blocks * block_size + (border * 2)) * scale as u32,
        (num_blocks * block_size + (border * 2)) * scale as u32,
    );

    println!("Create imagebuffer with size: {:?}", real_size);
    let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    for ((x, y), color) in pixels {
        let pixel = image::Rgb([color.0, color.1, color.2]);
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

    img.save(format!("frames/day20.frame{:05}.png", frame))
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    //println!("Part2: {}", solve_part2(args[1].to_string()));
}
