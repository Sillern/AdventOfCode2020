use image::ImageBuffer;
use regex::Regex;
use std::collections::HashMap;
use std::env;

type Color = (u8, u8, u8);
type Coordinate = (i32, i32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Location {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug)]
struct Tile {
    tile_id: i32,
    location: Coordinate,
    pixels: Vec<Coordinate>,
    borders: Vec<((i32, i32), Location)>,
    block_size: usize,
    dimension: usize,
}

impl Tile {
    fn new(block_index: usize, block_size: usize, block: &str) -> Tile {
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

        let dimension = 10;

        Tile {
            tile_id: tile_id,
            location: (
                (block_index % block_size) as i32,
                (block_index / block_size) as i32,
            ),
            borders: Tile::get_borders(&pixels, dimension),
            pixels: pixels,
            block_size: block_size,
            dimension: dimension,
        }
    }

    fn get_border_bitpattern(
        pixels: &Vec<Coordinate>,
        border: &Vec<Coordinate>,
        dimension: usize,
    ) -> (i32, i32) {
        let max_index = dimension as i32 - 1;

        let mut bitpattern = 0;
        let mut bitpattern_reversed = 0;
        border.iter().enumerate().for_each(|(bit, coord)| {
            if pixels.contains(&coord) {
                bitpattern_reversed |= 1 << bit;
                bitpattern |= 1 << (max_index - bit as i32);
            }
        });

        (bitpattern, bitpattern_reversed)
    }

    fn get_borders(pixels: &Vec<Coordinate>, dimension: usize) -> Vec<((i32, i32), Location)> {
        let max_index = dimension as i32 - 1;

        let top_row = (0..dimension as i32)
            .map(|x| (x, 0))
            .collect::<Vec<Coordinate>>();
        let bottom_row = (0..dimension as i32)
            .map(|x| (x, max_index))
            .collect::<Vec<Coordinate>>();
        let left_column = (0..dimension as i32)
            .map(|y| (0, y))
            .collect::<Vec<Coordinate>>();
        let right_column = (0..dimension as i32)
            .map(|y| (max_index, y))
            .collect::<Vec<Coordinate>>();

        vec![
            (
                Tile::get_border_bitpattern(pixels, &top_row, dimension),
                Location::Top,
            ),
            (
                Tile::get_border_bitpattern(pixels, &bottom_row, dimension),
                Location::Bottom,
            ),
            (
                Tile::get_border_bitpattern(pixels, &left_column, dimension),
                Location::Left,
            ),
            (
                Tile::get_border_bitpattern(pixels, &right_column, dimension),
                Location::Right,
            ),
        ]
    }

    fn draw(
        &self,
        tiles: &HashMap<i32, Tile>,
        pixels: &mut Vec<(Coordinate, Color)>,
        top_left: (i32, i32),
        tile_border: i32,
    ) {
        let color = (0x56, 0x67, 0x44);
        let border_color = (0x33, 0xCC, 0x66);
        let edge_color = (0x55, 0x48, 0x96);
        let corner_color = (0x80, 0x57, 0x88);

        for y in 0..self.dimension {
            for x in 0..self.dimension {
                if self.pixels.contains(&(x as i32, y as i32)) {
                    let pixel_color = if x == 0
                        || y == 0
                        || x as i32 == (self.dimension as i32 - 1)
                        || y as i32 == (self.dimension as i32 - 1)
                    {
                        border_color
                    } else {
                        match self.count_valid_borders(&tiles) {
                            2 => corner_color,
                            3 => edge_color,
                            _ => color,
                        }
                    };
                    pixels.push((
                        (
                            (top_left.0 + tile_border + x as i32),
                            (top_left.1 + tile_border + y as i32),
                        ),
                        pixel_color,
                    ));
                }
            }
        }
    }

    fn matches_border(&self, other_border: i32) -> bool {
        self.borders
            .iter()
            .any(|((border, _), _)| *border == other_border)
    }

    fn matching_border(&self, other_tile: &Tile) -> Option<(Location, bool)> {
        self.borders
            .iter()
            .filter_map(|((border, border_reversed), direction)| {
                if other_tile.tile_id != self.tile_id {
                    if other_tile.matches_border(*border) {
                        Some((*direction, false))
                    } else if other_tile.matches_border(*border_reversed) {
                        Some((*direction, true))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .next()
    }

    fn valid_borders(&self, tiles: &HashMap<i32, Tile>) -> Vec<i32> {
        tiles
            .iter()
            .filter_map(|(other_tile_id, other_tile)| {
                if self.borders.iter().any(|((border, border_reversed), _)| {
                    if *other_tile_id != self.tile_id {
                        other_tile.matches_border(*border)
                            || other_tile.matches_border(*border_reversed)
                    } else {
                        false
                    }
                }) {
                    Some(*other_tile_id)
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>()
    }

    fn count_valid_borders(&self, tiles: &HashMap<i32, Tile>) -> usize {
        self.valid_borders(&tiles).len()
    }

    fn is_edgepiece(&self, tiles: &HashMap<i32, Tile>) -> bool {
        // Edge pieces has three valid borders
        self.count_valid_borders(tiles) == 3
    }

    fn is_cornerpiece(&self, tiles: &HashMap<i32, Tile>) -> bool {
        // Corner pieces has two valid borders
        self.count_valid_borders(tiles) == 2
    }

    fn find_bordering_tiles(&self, tiles: &HashMap<i32, Tile>) -> Vec<i32> {
        self.valid_borders(&tiles)
    }

    fn rotate(&mut self) {
        self.pixels = self
            .pixels
            .iter()
            .map(|(x, y)| (9 - 1 * *y as i32, *x as i32))
            .collect::<Vec<Coordinate>>();
        self.borders = Tile::get_borders(&self.pixels, self.dimension);
    }

    fn flip_horizontal(&mut self) {
        self.pixels = self
            .pixels
            .iter()
            .map(|(x, y)| (9 - *x as i32, *y as i32))
            .collect::<Vec<Coordinate>>();
        self.borders = Tile::get_borders(&self.pixels, self.dimension);
    }

    fn flip_vertical(&mut self) {
        self.pixels = self
            .pixels
            .iter()
            .map(|(x, y)| (*x as i32, 9 - *y as i32))
            .collect::<Vec<Coordinate>>();
        self.borders = Tile::get_borders(&self.pixels, self.dimension);
    }

    fn remove_borders(&mut self) {
        self.pixels = self
            .pixels
            .iter()
            .filter_map(|&(x, y)| {
                if x == 0
                    || y == 0
                    || x == self.dimension as i32 - 1
                    || y == self.dimension as i32 - 1
                {
                    None
                } else {
                    Some((x as i32 - 1, y as i32 - 1))
                }
            })
            .collect::<Vec<Coordinate>>();
        self.dimension -= 2;
        self.borders = vec![];
    }
}

fn place_tile_next_to(
    current_tile_id: i32,
    other_tile_id: i32,
    tiles: &HashMap<i32, Tile>,
    block_size: usize,
) -> Option<Coordinate> {
    let current_tile = &tiles.get(&current_tile_id).unwrap();
    let other_tile = &tiles.get(&other_tile_id).unwrap();

    let next_location = match current_tile.matching_border(other_tile).unwrap().0 {
        Location::Top => (current_tile.location.0, current_tile.location.1 - 1),
        Location::Bottom => (current_tile.location.0, current_tile.location.1 + 1),
        Location::Left => (current_tile.location.0 - 1, current_tile.location.1),
        Location::Right => (current_tile.location.0 + 1, current_tile.location.1),
    };

    if next_location.0 < 0
        || next_location.1 < 0
        || next_location.0 > block_size as i32
        || next_location.1 > block_size as i32
    {
        None
    } else {
        Some(next_location)
    }
}

fn needs_to_flip(
    current_tile_id: i32,
    other_tile_id: i32,
    tiles: &HashMap<i32, Tile>,
) -> Option<Location> {
    let current_tile = &tiles.get(&current_tile_id).unwrap();
    let other_tile = &tiles.get(&other_tile_id).unwrap();

    let (other_direction, other_reversed) = other_tile.matching_border(current_tile).unwrap();

    if other_reversed {
        println!("Flip!: {:?}", other_direction);
        Some(other_direction)
    } else {
        None
    }
}

fn is_placed_correctly(
    current_tile_id: i32,
    other_tile_id: i32,
    tiles: &HashMap<i32, Tile>,
) -> bool {
    let current_tile = &tiles.get(&current_tile_id).unwrap();
    let other_tile = &tiles.get(&other_tile_id).unwrap();

    let (other_direction, other_reversed) = other_tile.matching_border(current_tile).unwrap();
    let (direction, reversed) = current_tile.matching_border(other_tile).unwrap();

    !other_reversed
        && match direction {
            Location::Top => Location::Bottom == other_direction,
            Location::Bottom => Location::Top == other_direction,
            Location::Left => Location::Right == other_direction,
            Location::Right => Location::Left == other_direction,
        }
}

fn switch_tiles(tiles: &mut HashMap<i32, Tile>, from: Coordinate, to: Coordinate) {
    let tiles_to_switch = tiles
        .iter()
        .filter_map(|(tile_id, tile)| {
            if tile.location == from || tile.location == to {
                Some(*tile_id)
            } else {
                None
            }
        })
        .collect::<Vec<i32>>();

    tiles_to_switch.iter().for_each(|&index| {
        tiles.entry(index).and_modify(|entry| {
            if entry.location == from {
                entry.location = to;
            } else if entry.location == to {
                entry.location = from;
            }
        });
    });
}

fn remove_borders(tiles: &mut HashMap<i32, Tile>) {
    tiles
        .iter_mut()
        .for_each(|(tile_id, tile)| tile.remove_borders());
}

fn visualize(tiles: &HashMap<i32, Tile>, block_size: usize, tile_border: i32, frame: i32) {
    let max_dimension: u32 = 10;
    let scale: u32 = 8;
    let border: u32 = 2;
    let tile_size = (max_dimension as i32 + tile_border * 2) as u32;
    let mut pixels = Vec::<(Coordinate, Color)>::new();

    println!("frame: {}, ", frame);

    tiles.iter().for_each(|(_, tile)| {
        tile.draw(
            &tiles,
            &mut pixels,
            (
                (tile.location.0 as usize * tile_size as usize) as i32,
                (tile.location.1 as usize * tile_size as usize) as i32,
            ),
            tile_border,
        );
    });

    let real_size = (
        (block_size as u32 * tile_size + border * 2) * scale as u32,
        (block_size as u32 * tile_size + border * 2) * scale as u32,
    );

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

fn visualize_full_map(pixels: &Vec<Coordinate>, full_size: usize, frame: i32) {
    let scale: u32 = 8;
    let border: u32 = 2;
    let real_size = (
        (full_size as u32 + border * 2) * scale as u32,
        (full_size as u32 + border * 2) * scale as u32,
    );

    let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    let color = (0x56, 0x67, 0x44);
    for &(x, y) in pixels {
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

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let block_size = (contents.split("\n\n").count() as f32).sqrt() as usize;
    let mut tiles = HashMap::<i32, Tile>::new();

    contents
        .split("\n\n")
        .enumerate()
        .for_each(|(index, tile_block)| {
            let tile = Tile::new(index, block_size, tile_block);
            tiles.entry(tile.tile_id).or_insert(tile);
        });

    tiles.iter().fold(1, |product, (tile_id, tile)| {
        if tile.is_cornerpiece(&tiles) {
            product * *tile_id as usize
        } else {
            product
        }
    })
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");
    let block_size = (contents.split("\n\n").count() as f32).sqrt() as usize;
    let mut tiles = HashMap::<i32, Tile>::new();

    contents
        .split("\n\n")
        .enumerate()
        .for_each(|(index, tile_block)| {
            let tile = Tile::new(index, block_size, tile_block);
            tiles.entry(tile.tile_id).or_insert(tile);
        });

    let enable_visualization = true;
    let tile_border = 1;
    let mut frame = 0;

    visualize(&tiles, block_size, tile_border, frame);

    let cornerpiece_id = tiles
        .iter()
        .filter_map(|(tile_id, tile)| {
            if tile.is_cornerpiece(&tiles) {
                Some(*tile_id)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    println!(
        "cornerpiece: {}: {:?}",
        cornerpiece_id,
        tiles.get(&cornerpiece_id).unwrap().location
    );

    // Place a cornerpiece at the top left tile
    let mut tiles_to_move = vec![(cornerpiece_id, (0, 0))];
    let mut completed_tiles = vec![];

    while tiles_to_move.len() > 0 {
        println!("Movelist: {:?}", tiles_to_move);
        println!("Completed list: {:?}", completed_tiles);
        let (tile_id_to_move, wanted_location) = tiles_to_move.pop().unwrap();

        let bordering_tiles = tiles
            .get(&tile_id_to_move)
            .unwrap()
            .find_bordering_tiles(&tiles);

        let location = tiles.get(&tile_id_to_move).unwrap().location;
        if location != wanted_location {
            println!("Moving tile from {:?} to {:?}", location, wanted_location);
            switch_tiles(&mut tiles, location, wanted_location);

            if enable_visualization {
                frame += 1;
                visualize(&tiles, block_size, tile_border, frame)
            }
        }

        bordering_tiles.iter().for_each(|bordering_tile_id| {
            if !completed_tiles.contains(bordering_tile_id) {
                let mut placed_bordering_tile = false;
                while !placed_bordering_tile {
                    let bordering_tile_location = tiles.get(&bordering_tile_id).unwrap().location;

                    match place_tile_next_to(
                        tile_id_to_move,
                        *bordering_tile_id,
                        &mut tiles,
                        block_size,
                    ) {
                        Some(new_location) => {
                            switch_tiles(&mut tiles, bordering_tile_location, new_location);
                            if !completed_tiles.contains(bordering_tile_id)
                                && !tiles_to_move.contains(&(*bordering_tile_id, new_location))
                            {
                                tiles_to_move.push((*bordering_tile_id, new_location));
                            }
                            placed_bordering_tile = true;
                        }
                        None => {
                            println!("!!!!!!!!!!!!!!1 {}: OutOfBounds, rotating", tile_id_to_move);
                            tiles.get_mut(&tile_id_to_move).unwrap().rotate();
                        }
                    }
                    if enable_visualization {
                        frame += 1;
                        visualize(&tiles, block_size, tile_border, frame)
                    }
                }

                while !is_placed_correctly(tile_id_to_move, *bordering_tile_id, &mut tiles) {
                    println!("{}: Misplaced, frame: {}", bordering_tile_id, frame);
                    match needs_to_flip(tile_id_to_move, *bordering_tile_id, &mut tiles) {
                        Some(direction) => match direction {
                            Location::Top | Location::Bottom => {
                                println!("flipping horizontal");
                                tiles.get_mut(bordering_tile_id).unwrap().flip_horizontal();
                            }
                            Location::Left | Location::Right => {
                                println!("flipping vertical");
                                tiles.get_mut(bordering_tile_id).unwrap().flip_vertical();
                            }
                        },
                        None => {
                            println!("rotating");
                            tiles.get_mut(bordering_tile_id).unwrap().rotate();
                        }
                    }

                    if enable_visualization {
                        frame += 1;
                        visualize(&tiles, block_size, tile_border, frame)
                    }
                }
            }
        });
        completed_tiles.push(tile_id_to_move);
    }

    frame += 1;
    visualize(&tiles, block_size, tile_border, frame);

    remove_borders(&mut tiles);

    for remove_border in 0..2 {
        frame += 1;
        visualize(&tiles, block_size, tile_border - remove_border, frame);
    }

    let mut all_pixels = Vec::<Coordinate>::new();
    tiles.iter().for_each(|(_, tile)| {
        tile.pixels.iter().for_each(|&(x, y)| {
            all_pixels.push((
                (tile.location.0 * tile.dimension as i32 + x),
                (tile.location.1 * tile.dimension as i32 + y),
            ));
        })
    });

    let pixel_max_x = all_pixels
        .iter()
        .fold(0, |max, &(x, _)| if x > max { x } else { max });
    let pixel_max_y = all_pixels
        .iter()
        .fold(0, |max, &(_, y)| if y > max { y } else { max });

    frame += 1;
    println!(
        "Visualizing full map on frame {}, (width: {})",
        frame, pixel_max_x
    );
    visualize_full_map(&all_pixels, pixel_max_x as usize, frame);

    let seamonster = "
..................#..
#....##....##....###.
.#..#..#..#..#..#....";

    let mut pattern = Vec::<Coordinate>::new();

    seamonster.lines().skip(1).enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, value)| {
            let coord = (x as i32, y as i32);
            if value != '.' {
                pattern.push(coord);
            }
        });
    });

    fn find_pattern(all_pixels: &Vec<Coordinate>, pattern: &Vec<Coordinate>) -> Vec<Coordinate> {
        let mut patterns_found = Vec::<Coordinate>::new();
        let pixel_max_x = all_pixels
            .iter()
            .fold(0, |max, &(x, _)| if x > max { x } else { max });
        let pixel_max_y = all_pixels
            .iter()
            .fold(0, |max, &(_, y)| if y > max { y } else { max });

        for y in 0..pixel_max_y {
            for x in 0..pixel_max_x {
                let check_coord = (x, y);

                let mut subpattern_found = Vec::<Coordinate>::new();

                if pattern.iter().all(|(pattern_x, pattern_y)| {
                    let pattern_coord = (x + pattern_x, y + pattern_y);
                    if all_pixels.contains(&pattern_coord) {
                        subpattern_found.push(pattern_coord);
                        true
                    } else {
                        false
                    }
                }) {
                    println!("Seamonster found at: {:?}", check_coord);
                    patterns_found.extend(subpattern_found);
                }
            }
        }
        patterns_found
    }

    let mut found_patterns = find_pattern(&all_pixels, &pattern);

    while found_patterns.len() == 0 {
        println!("Rotating");
        all_pixels = all_pixels
            .iter()
            .map(|(x, y)| (pixel_max_x - 1 * *y as i32, *x as i32))
            .collect::<Vec<Coordinate>>();

        frame += 1;
        visualize_full_map(&all_pixels, pixel_max_x as usize, frame);
        found_patterns = find_pattern(&all_pixels, &pattern);
    }

    all_pixels
        .iter()
        .filter(|coord| !found_patterns.contains(coord))
        .count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()),);
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
