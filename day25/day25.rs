use itertools::Itertools;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let public_keys = contents
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("public_keys: {:?}", public_keys);

    fn get_public_key(
        cache: &mut HashMap<(usize, usize), usize>,
        subject_number: usize,
        loop_size: usize,
    ) -> usize {
        let mut key = 1;
        let mut cached_loop_size = 0;
        for loop_size_in_cache in (0..loop_size).rev() {
            match cache.get(&(subject_number, loop_size_in_cache)) {
                Some(cached_key) => {
                    cached_loop_size = loop_size_in_cache;
                    key = *cached_key;
                    break;
                }
                None => (),
            }
        }

        for smaller_loop_size in cached_loop_size..loop_size {
            key = *cache
                .entry((subject_number, smaller_loop_size))
                .or_insert((key * subject_number) % 20201227);
        }

        key
    }

    fn reverse_public_key(
        cache: &mut HashMap<(usize, usize), usize>,
        public_key: &usize,
    ) -> (usize, usize) {
        let mut trials = 1;

        let subject_number = 7;
        loop {
            for loop_size in 0..trials {
                let trial_public_key = get_public_key(cache, subject_number, loop_size);

                if trial_public_key == *public_key {
                    return (subject_number, loop_size);
                }
            }
            trials *= 2;
            println!("Trials: {}, cache_size: {}", trials, cache.len());
        }
    }

    let mut cache = HashMap::<(usize, usize), usize>::new();
    let secrets = public_keys
        .iter()
        .map(|key| {
            let (subject_number, loop_size) = reverse_public_key(&mut cache, key);
            println!(
                "public key: {}, reversed: {:?}, ",
                key,
                (subject_number, loop_size)
            );
            (subject_number, loop_size)
        })
        .collect::<Vec<(usize, usize)>>();

    secrets.iter().for_each(|(subject_number, loop_size)| {
        println!(
            "secret: {}",
            get_public_key(&mut cache, *subject_number, *loop_size),
        );
    });

    println!("public_keys: {:?}, secrets: {:?}", public_keys, secrets);

    let encryption_key1 = get_public_key(&mut cache, public_keys[1], secrets[0].1);
    let encryption_key2 = get_public_key(&mut cache, public_keys[0], secrets[1].1);

    println!(
        "Validated encryption keys: {} == {}",
        encryption_key1, encryption_key2
    );
    encryption_key1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    //println!("Part2: {}", solve_part2(args[1].to_string()));
}
