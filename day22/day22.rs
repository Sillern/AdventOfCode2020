use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut player_cards = contents
        .split("\n\n")
        .map(|block| {
            let mut cards = VecDeque::<usize>::new();
            block
                .lines()
                .skip(1)
                .for_each(|line| cards.push_back(line.parse::<usize>().unwrap()));

            cards
        })
        .collect::<Vec<VecDeque<usize>>>();

    println!("players: {:?}", player_cards);

    while player_cards[0].len() != 0 && player_cards[1].len() != 0 {
        println!("player1: {:?}", player_cards[0]);
        println!("player2: {:?}", player_cards[1]);
        let player1 = player_cards[0].pop_front().unwrap();
        let player2 = player_cards[1].pop_front().unwrap();
        if player1 > player2 {
            player_cards[0].push_back(player1);
            player_cards[0].push_back(player2);
        } else {
            player_cards[1].push_back(player2);
            player_cards[1].push_back(player1);
        }
    }
    println!("player1: {:?}", player_cards[0]);
    println!("player2: {:?}", player_cards[1]);
    player_cards
        .iter()
        .map(|deck| {
            let deck_size = deck.len();

            deck.iter()
                .enumerate()
                .fold(0, |sum, (index, card)| sum + card * (deck_size - index))
        })
        .fold(0, |max, result| if result > max { result } else { max })
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut player_cards = contents
        .split("\n\n")
        .map(|block| {
            let mut cards = VecDeque::<usize>::new();
            block
                .lines()
                .skip(1)
                .for_each(|line| cards.push_back(line.parse::<usize>().unwrap()));

            cards
        })
        .collect::<Vec<VecDeque<usize>>>();

    println!("players: {:?}", player_cards);

    while player_cards[0].len() != 0 && player_cards[1].len() != 0 {
        println!("player1: {:?}", player_cards[0]);
        println!("player2: {:?}", player_cards[1]);
        let player1 = player_cards[0].pop_front().unwrap();
        let player2 = player_cards[1].pop_front().unwrap();
        if player1 > player2 {
            player_cards[0].push_back(player1);
            player_cards[0].push_back(player2);
        } else {
            player_cards[1].push_back(player2);
            player_cards[1].push_back(player1);
        }
    }
    println!("player1: {:?}", player_cards[0]);
    println!("player2: {:?}", player_cards[1]);
    player_cards
        .iter()
        .map(|deck| {
            let deck_size = deck.len();

            deck.iter()
                .enumerate()
                .fold(0, |sum, (index, card)| sum + card * (deck_size - index))
        })
        .fold(0, |max, result| if result > max { result } else { max })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
